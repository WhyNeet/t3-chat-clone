import { ChevronRight } from "lucide-react";
import { NavLink } from "react-router";

export function ThemesSettings() {
  return (
    <div className="h-full w-full flex items-center justify-center px-4 sm:px-8 md:px-10 lg:px-16 animate-in zoom-in-95 fade-in duration-200">
      <div className="max-w-3xl w-full">
        <div className="flex items-center gap-2 mb-4 font-display text-pink-950/40 text-sm font-medium">
          <NavLink
            to="/settings"
            className="text-pink-950/40! underline"
          >
            Settings
          </NavLink>{" "}
          <ChevronRight className="h-3 w-3 stroke-3 translate-y-[1px]" />
        </div>
        <h1 className="text-2xl font-bold font-display mb-4">Themes</h1>
        <div className="font-display font-medium text-[15px] text-pink-900/60">
          Coming soon.
        </div>
      </div>
    </div>
  );
}
