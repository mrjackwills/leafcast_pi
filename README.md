<p align="center">
 	THIS IS THE TYPESCRIPT BRANCH AND IS NO LONGER MAINTAINED
	<br>
	THIS IS THE TYPESCRIPT BRANCH AND IS NO LONGER MAINTAINED
	<br>
	THIS IS THE TYPESCRIPT BRANCH AND IS NO LONGER MAINTAINED
	<br>
</p>

<p align="center">
	<img src='./.github/logo.svg' width='200px'/>
</p>

<p align="center">
	<h1 align="center">Leafcast - pi client</h1>
</p>

<p align="center">
	The pi client for Leafcast, powered by <a href='https://www.staticpi.com' target='_blank' rel='noopener noreferrer'>staticPi.com</a>
</p>
<p align="center">
	Built in <a href='https://www.typescriptlang.org/' target='_blank' rel='noopener noreferrer'>Typescript</a>, for <a href='https://nodejs.org/en/' target='_blank' rel='noopener noreferrer'>Node.js</a>
</p>

<p align="center">
	Back end to the <a href='https://github.com/mrjackwills/leafcast_vue' target='_blank' rel='noopener noreferrer'>vue frontend website</a>
</p>


## Required services

1) <a href='https://www.staticpi.com/' target='_blank' rel='noopener noreferrer'>staticPi</a> - the simple and secure messaging service

## Required software

1) <a href='https://nodejs.org/en/' target='_blank' rel='noopener noreferrer'>Node.js</a> - runtime


## Suggested directories and file

| directory | reason|
| --- | --- |
|```~/leafcast/logs```			| Log files |
|```~/leafcast/photos```		| Photos, organised into dates, 2021-01-01 |
|```~/leafcast/client```		| code location |
|```~/leafcast/client/.env```	| enviromental variables, make sure in production mode! |

## Build step
1) ```npm run build```

## Run step
a) ```pm2 start pm2.config.js``` load up into pm2

*or*

b) ```node dist/index``` run in shell directly