const appName = [ 'leafcast' ];

const apps = [];
for (const i of appName) {
	apps.push({
		name: i,
		script: 'dist/index.js',
		node_args: '--trace-warnings',
		instances: 1,
		exec_mode: 'fork',
		watch: true,
		max_memory_restart: '150M'
	});
}

module.exports = {
	apps
};