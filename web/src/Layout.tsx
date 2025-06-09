import '@fontsource-variable/space-grotesk';
import '@fontsource-variable/geist';
import { Outlet } from "react-router";
import { init } from './lib/state';

init();

export function Layout() {
  return <main className="h-screen w-screen">
    <Outlet />
  </main>
}
