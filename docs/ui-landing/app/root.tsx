import type { LinksFunction, V2_MetaFunction } from '@remix-run/node';
import {
	Links,
	LiveReload,
	Meta,
	Outlet,
	Scripts,
	ScrollRestoration,
} from '@remix-run/react';
import styles from './styles/index.css';

export const links: LinksFunction = () => {
	return [{ rel: 'stylesheet', href: styles }];
};

export const meta: V2_MetaFunction = () => {
	return [
		{ title: 'Rapid UI' },
		{
			name: 'description',
			content:
				'A supercharged component library with global theming and beautiful unstyled components all built ontop of TailwindCSS.',
		},
	];
};

export default function App() {
	return (
		<html lang='en'>
			<head>
				<meta charSet='utf-8' />
				<meta
					name='viewport'
					content='width=device-width,initial-scale=1'
				/>
				<Meta />
				<Links />
			</head>
			<body>
				<Outlet />
				<ScrollRestoration />
				<Scripts />
				<LiveReload />
			</body>
		</html>
	);
}
