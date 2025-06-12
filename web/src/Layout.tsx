/* @ts-ignore */
import '@fontsource-variable/space-grotesk';
/* @ts-ignore */
import '@fontsource-variable/geist';
/* @ts-ignore */
import '@fontsource-variable/jetbrains-mono';
import { Outlet } from "react-router";
import { init } from './lib/state';

init();

export function Layout() {
  return <main className="h-screen w-screen">
    <Outlet />
  </main>
}
