module.exports = {
	preset: 'ts-jest',
	roots: [
		'<rootDir>/src'
	],
	testMatch: [
		'**/__tests__/**/*.+(ts|tsx|js)',
		'**/?(*.)+(spec|test).+(ts|tsx|js)'
	],
	transform: {
		'^.+\\.(ts|tsx)$': 'ts-jest'
	},
	testPathIgnorePatterns: [ './src/__tests__/testHelper.ts', 'jestSettings.ts', './src/__tests__/mocks/*' ],
	coveragePathIgnorePatterns: [ './src/__tests__/testHelper.ts', 'jestSettings.ts', './dist/*', './src/__tests__/mocks/*'],
	setupFilesAfterEnv: [ './src/__tests__/jestSettings.ts' ],

};