import { CopyIcon, ShareIcon } from "lucide-react"
import { Button } from "./ui/button"
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover"
import { useEffect, useState } from "react"
import { getShareState, shareChat, unshareChat } from "../lib/api/chats"
import { Loader } from "./ui/loader"
import type { Share } from "../lib/model/share"
import { cn } from "./utils"

export const ChatShare = ({ id }: { id: string | null }) => {
  return <Popover>
    <PopoverTrigger asChild>
      <Button
        intent="ghost"
        size="square"
        rounded="circle"
        className={cn("bg-pink-300/20 hover:bg-pink-400/20 backdrop-blur-2xl transition duration-200", id ? "opacity-100 scale-100" : "scale-50 opacity-0 pointer-events-none")}
      >
        <ShareIcon className="h-[18px] w-[18px] text-pink-900" />
      </Button>
    </PopoverTrigger>
    <PopoverContent className="border border-pink-900/20 bg-pink-50 px-4 py-3" align="end">
      {id ? <ChatShareState id={id} /> : null}
    </PopoverContent>
  </Popover>
}

export const ChatShareState = ({ id }: { id: string }) => {
  const [shareState, setShareState] = useState<Share | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const link = shareState ? ((import.meta.env.DEV ? "http://localhost:5173/" : "https://whychat.vercel.app/") + (`chat/${shareState.id}?id=${shareState.share_id}`)) : null;

  useEffect(() => {
    setIsLoading(true)
    getShareState(id).then(share => {
      setShareState(share);
      setIsLoading(false);
    }).catch(() => setIsLoading(false));
  }, [id]);

  const handleUnshareClick = async () => {
    if (!shareState) return;

    await unshareChat(id, shareState.share_id);
    setShareState(null);
  }

  const handleShareClick = async () => {
    const share = await shareChat(id);
    setShareState(share);
  };

  const handleCopyClick = () => {
    navigator.clipboard.writeText(link!);
  };

  if (isLoading) return <div className="text-sm font-display font-medium flex items-center gap-4 text-pink-900"><Loader className="text-pink-900 h-5 w-5" />Loading...</div>;

  return <div>
    <div className="font-medium font-display text-pink-900 mb-2">Share Chat</div>
    {shareState ? <div>
      <div className="border border-pink-900/20 bg-pink-100 text-pink-900 text-sm whitespace-nowrap rounded-lg px-2 py-1.5 relative overflow-hidden">
        <div className="max-w-full overflow-x-hidden overflow-ellipsis">{link}</div>
        <div className="h-full flex items-center absolute right-0 inset-y-0 pr-2 pl-8 -bg-linear-90 from-pink-100 from-50% to-transparent z-10">
          <button className="h-4 w-4 cursor-pointer" onClick={handleCopyClick}>
            <CopyIcon className="h-4 w-4" />
          </button>
        </div>
      </div>
      <Button onClick={handleUnshareClick} intent="secondary" className="w-full py-1.5 font-medium text-pink-900 hover:bg-pink-900/10 mt-2" size="small">Unshare</Button>
    </div> : <div className="">
      <Button onClick={handleShareClick} intent="primary" className="w-full py-2" size="small">Create Link</Button>
    </div>}
  </div>
}