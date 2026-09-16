async (page) => {
  const base = await page.evaluate(() => location.origin);
  const failures = [];
  page.on('pageerror', error => failures.push(error.message));
  const check = (condition, message) => {
    if (!condition) throw new Error(message);
  };
  const command = async value => {
    await page.getByRole('textbox', { name: 'Terminal command' }).fill(value);
    await page.getByRole('button', { name: 'Run ↵' }).click();
  };
  const output = page.getByRole('log', { name: 'Terminal output' });

  await page.goto(base);
  await page.getByRole('heading', { name: 'About me', exact: true }).waitFor();
  check(await page.getByRole('link', { name: 'Explore my work' }).count() === 1, 'Visual homepage must be the default');
  await page.getByRole('link', { name: 'Explore my work' }).click();
  await page.getByRole('heading', { name: 'Learning by building.' }).waitFor();
  await page.getByRole('link', { name: 'Putting the pieces back together', exact: true }).click();
  await page.getByRole('heading', { name: 'Putting the pieces back together', exact: true }).waitFor();
  await page.getByRole('link', { name: 'Read in terminal' }).click();
  await output.getByText('Putting the pieces back together', { exact: false }).waitFor();
  check(page.url().split('?')[0] === `${base}/terminal`, 'Project command must stay in terminal');
  check(await output.getByRole('link', { name: 'Explore the code' }).getAttribute('href') === 'https://github.com/skanjalkar/aries-rust', 'Repository link must match the project');
  await page.getByRole('link', { name: 'Browse', exact: true }).click();
  await page.getByRole('heading', { name: 'Putting the pieces back together', exact: true }).waitFor();
  await page.getByRole('link', { name: 'Terminal', exact: true }).click();
  check((await output.textContent()).includes('Database recovery'), 'Switching modes must preserve output');

  await command('echo repeat');
  await command('echo repeat');
  check(await output.locator('.terminal-text').filter({ hasText: /^repeat$/ }).count() === 2, 'Repeated output must appear twice');
  await command('clear');
  check(await output.locator('.terminal-entry').count() === 0, 'Clear must empty the screen');
  const input = page.getByRole('textbox', { name: 'Terminal command' });
  await input.focus();
  await input.press('ArrowUp');
  check(await input.inputValue() === 'clear', 'Clear must preserve command history');
  await input.fill('draft');
  await input.press('ArrowDown');
  await input.fill('draft');
  await input.press('ArrowUp');
  await input.press('ArrowDown');
  check(await input.inputValue() === 'draft', 'History must restore an unfinished draft');
  await input.fill('open ari');
  await input.press('Tab');
  check(await input.inputValue() === 'open aries', 'Project autocomplete must work');
  await input.press('Escape');
  await input.press('Tab');
  check(await page.getByRole('button', { name: 'Run ↵' }).evaluate(el => el === document.activeElement), 'Escape and Tab must let keyboard users leave input');
  await command('open aries');
  await command('projects');
  await output.getByRole('link', { name: 'open aries →', exact: true }).last().click();
  await output.getByRole('link', { name: 'open aries →', exact: true }).last().click();
  check(await output.getByRole('link', { name: 'Explore the code ↗', exact: true }).count() === 3, 'Repeated suggested links must execute every time');
  await command('contact');
  check(await output.getByRole('link', { name: 'LinkedIn ↗', exact: true }).getAttribute('href') === 'https://www.linkedin.com/in/shreyas1405/', 'Contact must use shared profile');
  await command('read about-me');
  await output.getByText('See ya! :)', { exact: true }).waitFor();
  check(await output.locator('img').count() === 6, 'Full article images must be available in terminal');
  await command('echo <img src=x onerror=alert(1)>');
  check(await output.locator('img').count() === 6, 'Input must render as text, not HTML');
  await command('not-a-command');
  check((await output.textContent()).includes('Command not found'), 'Unknown commands must explain recovery');
  await command('exit');
  await page.getByRole('heading', { name: 'Putting the pieces back together', exact: true }).waitFor();

  await page.goto(`${base}/about`);
  await page.getByRole('link', { name: 'Read my résumé' }).waitFor();
  check(await page.getByRole('link', { name: 'Read my résumé' }).getAttribute('rel') === 'external', 'Résumé must bypass client routing');
  const resume = await page.request.get(`${base}/resume/resume.pdf`);
  check(resume.ok() && resume.headers()['content-type'].includes('pdf'), 'Résumé must be served as a PDF');
  await page.goto(`${base}/blog/about-me`);
  await page.getByRole('heading', { name: 'About me', exact: true }).waitFor();
  check((await page.locator('main').textContent()).includes('From the archive'), 'Historical writing must be dated');
  await page.goto(`${base}/not-a-page`);
  await page.getByRole('heading', { name: 'Nothing on this shelf.' }).waitFor();

  await page.emulateMedia({ reducedMotion: 'reduce' });
  for (const width of [320, 390, 768, 1440]) {
    await page.setViewportSize({ width, height: 900 });
    for (const path of ['/', '/projects', '/about', '/blog', '/terminal']) {
      await page.goto(`${base}${path}`);
      await page.locator('main h1').waitFor();
      const fits = await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth);
      check(fits, `Horizontal overflow at ${width}px on ${path}`);
    }
  }
  check(failures.length === 0, `Browser errors: ${failures.join('; ')}`);
  await page.goto(base);
  return { passed: true, checks: 'Visual navigation, shared content, mode retention, repeated output and links, history, completion, keyboard escape, article reading, HTML escaping, error recovery, résumé, direct routes, 20 responsive layouts', browserErrors: failures };
}
