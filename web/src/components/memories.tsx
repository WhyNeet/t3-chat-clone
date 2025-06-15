import { Database, XIcon } from "lucide-react";
import { Button } from "./ui/button";
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger } from "./ui/dialog";
import { useMemoryStore } from "../lib/state/memory";
import { Loader } from "./ui/loader";
import { AuthState, useAuthStore } from "../lib/state/auth";
import { NavLink } from "react-router";
import { Fragment } from "react/jsx-runtime";
import { removeMemory } from "../lib/api/service";

export const Memories = () => {
  const isLoggedIn = useAuthStore(state => state.state === AuthState.Loaded && state.user !== null);
  const memories = useMemoryStore(state => state.memories);
  const removeMemoryState = useMemoryStore(state => state.removeMemory);

  const handleRemoveMemory = async (id: string) => {
    await removeMemory(id);
    removeMemoryState(id);
  }

  return <Dialog>
    <DialogTrigger asChild>
      <Button
        onClick={() => { }}
        intent="ghost"
        size="square"
        rounded="circle"
        className="bg-pink-300/20 hover:bg-pink-400/20 backdrop-blur-2xl"
      >
        <Database className="h-[18px] w-[18px] text-pink-900" />
      </Button>
    </DialogTrigger>
    <DialogContent className="pb-0">
      <DialogHeader>
        <DialogTitle className="text-xl font-medium font-display mb-0">Memories</DialogTitle>
        <DialogDescription className="text-sm font-display text-pink-900">
          AI automatically creates memories about you. It may use them later to respond to your messages.
        </DialogDescription>
      </DialogHeader>
      {isLoggedIn ? (memories ? <div className="flex flex-col gap-2 max-h-[70vh] overflow-y-scroll scrollbar-none pb-6">
        {memories.length > 0 ? memories.map(memory => <Fragment key={memory.id}>
          <div className="w-fit max-w-5/6 font-display px-3 py-2 text-pink-900 flex items-center justify-between rounded-lg border border-pink-900/20 bg-pink-50 group relative">
            {memory.content}
            <button onClick={() => handleRemoveMemory(memory.id)} className="cursor-pointer text-pink-200 brightness-75 absolute -right-7 inset-y-0">
              <XIcon className="h-[18px] w-[18px]" />
            </button>
          </div>
        </Fragment>) : <div className="font-display text-pink-900">No memories recorded yet.</div>}
      </div> : <div className="flex items-center gap-4 font-display font-medium text-pink-900/60 text-sm pb-4">
        <Loader className="h-6 w-6" />
        Loading memories...
      </div>) : <div className="font-display font-medium text-pink-900 pb-4">
        <NavLink to="/auth/login">Log In</NavLink> to use this feature.
      </div>}
    </DialogContent>
  </Dialog>;
}