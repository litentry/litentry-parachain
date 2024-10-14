/* eslint-disable */
export default {
  displayName: 'vc-sdk',
  preset: '../../jest.preset.js',
  testEnvironment: 'node',
  transform: {
    '^.+\\.[tj]s$': ['ts-jest', { tsconfig: '<rootDir>/tsconfig.spec.json' }],
  },
  moduleFileExtensions: ['ts', 'js', 'html'],
  coverageDirectory: '../../coverage/package/vc-sdk',
  transformIgnorePatterns: ['node_modules/(?!(@litentry/|.pnpm/@litentry)).*'],
  moduleNameMapper: {},
};
