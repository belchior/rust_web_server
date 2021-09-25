module.exports = {
  'env': {
    'es6': true,
    'browser': true,
    'jest/globals': true
  },
  'plugins': [
    'jest',
  ],
  'extends': [
    'react-app',
    'eslint:recommended',
    'plugin:@typescript-eslint/eslint-recommended',
    'plugin:jest/recommended'
  ],
  'globals': {
    'Atomics': 'readonly',
    'SharedArrayBuffer': 'readonly',
  },
  'parser': '@typescript-eslint/parser',
  'parserOptions': {
    'ecmaVersion': 2018,
    'sourceType': 'module'
  },
  'rules': {
    'indent': ["error", 2, { SwitchCase: 1 }],
    'linebreak-style': ['error', 'unix'],
    'max-len': ['error', { code: 120, ignoreComments: true }],
    // 'no-console': ["error", { allow: ["warn", "error"] }],
    'no-console': 'off',
    'object-curly-spacing': ['error', 'always'],
    'quotes': ['error', 'single'],
    'semi': ['error', 'always'],
    'no-unused-vars': 'off',
  }
};
