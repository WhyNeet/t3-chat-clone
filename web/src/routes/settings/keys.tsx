import { Check, ChevronRight, Plus, X } from "lucide-react";
import { AuthState, useAuthStore } from "../../lib/state/auth";
import { NavLink } from "react-router";
import { useState } from "react";
import { Input } from "../../components/ui/input";
import { Button } from "../../components/ui/button";
import { useServiceStore } from "../../lib/state/service";
import { Loader } from "../../components/ui/loader";
import { deleteKey, enrollKey } from "../../lib/api/keys";

export function KeysSettings() {
  const isAuthorized = useAuthStore((state) => state.user !== null);
  const authLoading = useAuthStore(
    (state) => state.state === AuthState.Loading,
  );
  const keys = useServiceStore(state => state.keys);
  const deleteKeyState = useServiceStore(state => state.removeKey);
  const addKeyState = useServiceStore(state => state.addKey);
  const [isAddingKey, setIsAddingKey] = useState(false);
  const [key, setKey] = useState("");

  const handleKeyDelete = async (id: string) => {
    await deleteKey(id);
    deleteKeyState(id);
  }

  const handleKeyEnroll = async () => {
    if (!key.length) return;
    setIsAddingKey(false);
    const keyId = await enrollKey(key, "openrouter");
    setKey("");
    addKeyState({ id: keyId, provider: "openrouter" });
  }

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
        <h1 className="text-2xl font-bold font-display">Keys</h1>
        <p className="mb-4 text-pink-900 font-display">Add your own key to access paid models.</p>
        {authLoading ? <div>
          <Loader className="text-pink-900" />
        </div> : isAuthorized ? keys ? (
          <>
            <div className="text-[15px]">
              {keys.map(key =>
                <div key={key.id} className="font-display px-3 py-2 pb-3 rounded-lg text-pink-900 bg-pink-50 w-full animate-in zoom-in-95 fade-in relative">
                  <button onClick={() => handleKeyDelete(key.id)} className="absolute top-3 right-3 cursor-pointer">
                    <X className="h-5 w-5" />
                  </button>
                  <div className="font-medium mb-2">OpenRouter Key</div>
                  <p>Key ID: {key.id}</p>
                </div>
              )}
              {isAddingKey ? (
                <div className="font-display px-3 py-2 pb-3 rounded-lg text-pink-900 bg-pink-50 w-full animate-in zoom-in-95 fade-in">
                  <div className="font-medium mb-2">OpenRouter Key</div>
                  <div className="flex items-center gap-2">
                    <Input value={key} onChange={(e) => setKey(e.currentTarget.value.trim())} className="py-2 text-sm w-full focus:border-pink-900" placeholder="Your OpenRouter API Key..." />
                    <Button disabled={key.length === 0} className="text-pink-900 disabled:opacity-60 disabled:cursor-not-allowed" onClick={() => handleKeyEnroll()}>
                      <Check className="h-5 w-5" />
                    </Button>
                  </div>
                </div>
              ) : null}
              {isAddingKey || keys.length > 0 ? null : <button
                onClick={() => setIsAddingKey(true)}
                className="font-display px-3 py-2 rounded-lg text-pink-900 hover:bg-pink-50 cursor-pointer text-left w-full flex items-center gap-2 active:scale-[0.97] transition-[scale]"
              >
                <Plus className="h-4 w-4" />
                Add OpenRouter Key
              </button>}
              <p className="font-display text-sm mt-6 text-pink-900/60">Your keys are encrypted and stored in primary database and cache.</p>
            </div>
          </>
        ) : <div>
          <Loader className="text-pink-900" />
        </div> : (
          <>
            <div className="font-display font-medium text-[15px] text-pink-900/60">
              <NavLink to="/auth/login" className="mr-[5px]">
                Log In
              </NavLink>
              to use this feature.
            </div>
          </>
        )}
      </div>
    </div>
  );
}
