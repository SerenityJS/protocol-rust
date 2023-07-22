// Runs after napi to dynamically inject abstracted types
const fs = require('fs');
const path = require('path');

const FilePath = path.join(__dirname, 'index.d.ts');

// Recommended use template bierner.comment-tagged-templates
const injection = /* ts */ `
// Injected types by build.js
`; 

fs.readFile(FilePath, 'utf8', (err, data) => {
  if (err) throw err;
  const result = injection + '\n' + 'data';
  fs.writeFile(FilePath, result, 'utf8', (err) => {
    if (err) throw err;

    console.log('build.js: Injected extra types into index.d.ts')
  });
});
