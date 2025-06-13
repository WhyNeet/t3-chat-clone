import { Fragment } from "react/jsx-runtime";
import { Key, SwatchBook } from "lucide-react";
import { useNavigate } from "react-router";

export function Settings() {
  const navigate = useNavigate();

  return (
    <div className="h-full w-full flex items-center justify-center px-4 sm:px-8 md:px-10 lg:px-16 animate-in zoom-in-95 fade-in duration-200">
      <div className="max-w-3xl w-full">
        <h1 className="text-2xl font-bold font-display mb-6">Settings</h1>
        <div className="text-[15px] w-full">
          {[
            { name: "Themes", path: "themes", icon: SwatchBook },
            { name: 'Keys', path: "keys", icon: Key }
          ].map(({ name, path, icon: Icon }) => (
            <Fragment key={name}>
              <button
                onClick={() => navigate(path)}
                className="font-display px-3 py-2 rounded-lg text-pink-900 hover:bg-pink-50 cursor-pointer text-left w-full flex items-center gap-2 active:scale-[0.97] transition"
              >
                <Icon className="h-4 w-4" />
                {name}
              </button>
              <hr className="last:hidden my-1 text-pink-900/10" />
            </Fragment>
          ))}
        </div>
      </div>
    </div>
  );
}
