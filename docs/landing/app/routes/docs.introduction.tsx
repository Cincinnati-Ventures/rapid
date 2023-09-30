import React, { useState, useEffect } from 'react';
import { docsSetup } from '~/helpers';
import { Heading, Text } from '@rapid-web/ui';
import type { LoaderFunction, LinksFunction } from '@remix-run/node';
import { useLoaderData, Outlet, Link } from '@remix-run/react';
import { BreadCrumb } from '~/components/BreadCrumb';
import styles from '../styles/markdown.css';
import Github from '../../assets/github.svg';
import { faChevronRight } from '@fortawesome/free-solid-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { getNextDocPathName, shouldShowDocsNavigation } from './docs';

interface LoaderOutput {
	routes: string[];
}

export const links: LinksFunction = () => {
	return [
		{
			rel: 'stylesheet',
			href: styles,
		},
	];
};

export const loader: LoaderFunction = ({ request }) => {
	return docsSetup('introduction', request);
};

const DocsIntroduction = () => {
	const data = useLoaderData<LoaderOutput>();
	const [pathName, setPathName] = useState<string>();
	const [isShowingDocsNavigation, setIsShowingDocsNavigation] =
		useState(false);

	useEffect(() => {
		setPathName(window.location.pathname);
		setIsShowingDocsNavigation(
			shouldShowDocsNavigation(pathName as string),
		);
	}, [pathName]);
	return (
		<div className='flex w-full flex-col'>
			<BreadCrumb routes={data.routes} />
			<Heading styles='exclude-from-markdown text-white text-5xl font-bold'>
				Introduction
			</Heading>
			<div className='mt-6 text-white'>
				<Outlet />
			</div>
			{isShowingDocsNavigation && (
				<div className='mt-16 flex flex-col items-center gap-4 md:flex-row'>
					<a
						href='https://github.com/Cincinnati-Ventures/rapid'
						className='exclude-from-markdown w-full no-underline md:w-1/3'
					>
						<div className='hover:border-mainBlue flex items-center gap-2 rounded-lg border-[0.5px] border-[#222222] p-4 transition-all duration-100 ease-linear'>
							<img width={24} src={Github} alt='github' />
							<Text className='text-sm font-bold text-white'>
								View on Github
							</Text>
						</div>
					</a>
					<Link
						to={getNextDocPathName(pathName).path}
						className='exclude-from-markdown w-full no-underline md:w-1/3'
					>
						<div className='hover:border-mainBlue flex items-center gap-2 rounded-lg border-[0.5px] border-[#222222] p-4 transition-all duration-100 ease-linear'>
							<FontAwesomeIcon
								icon={faChevronRight}
								color='white'
								width={20}
								height={20}
							/>
							<Text className='text-sm font-bold text-white'>
								Next Doc: {getNextDocPathName(pathName)?.text}
							</Text>
						</div>
					</Link>
				</div>
			)}
		</div>
	);
};

export default DocsIntroduction;
