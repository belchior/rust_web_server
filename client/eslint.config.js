import js from "@eslint/js";
import globals from "globals";
import reactHooks from "eslint-plugin-react-hooks";
import reactRefresh from "eslint-plugin-react-refresh";
import tseslint from "typescript-eslint";
import { defineConfig, globalIgnores } from "eslint/config";
import pluginJest from "eslint-plugin-jest";

export default defineConfig([
  globalIgnores(["dist"]),
  {
    files: ["**/*.{ts,tsx}"],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      ecmaVersion: 2020,
      globals: {
        ...globals.browser,
        ...pluginJest.environments.globals.globals,
      },
    },
    plugins: { jest: pluginJest },
    rules: {
      "@typescript-eslint/no-unused-vars": [
        "error",
        { argsIgnorePattern: "^_", caughtErrorsIgnorePattern: "^_" },
      ],
      "comma-dangle": [
        "error",
        {
          arrays: "always-multiline",
          objects: "always-multiline",
          imports: "always-multiline",
          exports: "always-multiline",
          functions: "never",
        },
      ],
      indent: ["error", 2, { SwitchCase: 1 }],
      "jest/no-disabled-tests": "warn",
      "jest/no-focused-tests": "error",
      "jest/no-identical-title": "error",
      "jest/prefer-to-have-length": "warn",
      "jest/valid-expect": "error",
      "linebreak-style": ["error", "unix"],
      "max-len": [
        "error",
        {
          code: 100,
          tabWidth: 2,
          ignoreComments: true,
          ignoreTrailingComments: true,
          ignoreUrls: true,
          ignoreStrings: true,
          ignoreTemplateLiterals: true,
        },
      ],
      "no-console": "error",
      "no-multiple-empty-lines": ["error", { max: 1 }],
      "no-unused-vars": "off",
      "object-curly-spacing": ["error", "always"],
      "padded-blocks": ["error", "never"],
      quotes: ["error", "single"],
      semi: [2, "never"],
      "space-before-function-paren": [
        "error",
        { anonymous: "always", asyncArrow: "always", named: "never" },
      ],
      "space-in-parens": ["error", "never"],
      "jsx-quotes": ["error", "prefer-double"],
      "eol-last": ["error", "always"],
    },
  },
]);
