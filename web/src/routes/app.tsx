import { useState } from "react";
import { Button } from "../components/ui/button";
import { ChatsSidebar } from "../components/chats-sidebar";
import { ChevronRight, Command, SendHorizonal } from "lucide-react";
import { Outlet, useParams } from "react-router";
import { createMessage } from "../lib/api/messages";
import { subscribeToStream } from "../lib/api/completions";
import { createChat } from "../lib/api/chats";
import { useChatsStore } from "../lib/state/chats";
import { useNavigate } from "react-router";

export function App() {
  const navigate = useNavigate();
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [message, setMessage] = useState("");
  const updatePendingMessage = useChatsStore(
    (state) => state.updatePendingMessage,
  );
  const addMessages = useChatsStore(state => state.addChatMessages);
  const addChat = useChatsStore((state) => state.initializeChat);
  const params = useParams();
  const chatId = params["chatId"];

  const sendMessage = async () => {
    if (!chatId) {
      const chat = await createChat();
      addChat(chat);
      navigate(`/chat/${chat.id}`);
      await sendAndSubscribe(chat.id);
    } else {
      navigate(`/chat/${chatId}`);
      await sendAndSubscribe(chatId);
    }
  };

  const sendAndSubscribe = async (chatId: string) => {
    const { stream_id, user_message } = await createMessage(chatId, {
      message,
      model: "google/gemini-2.0-flash-exp:free",
    });
    addMessages(chatId, [user_message]);
    subscribeToStream(stream_id, (delta) => {
      updatePendingMessage(chatId, delta.content);
    });
  };

  return (
    <div
      className={`h-full w-full flex items-stretch transition-all relative duration-200 bg-pink-200 ${sidebarOpen ? "pt-1" : ""}`}
    >
      <div
        className={`transition-all duration-200 ease-in-out relative ${sidebarOpen ? "min-w-72 w-72" : "min-w-0 w-0"}`}
      >
        <ChatsSidebar />
      </div>
      <div
        className={`h-full w-full bg-white transition-all duration-200 ${sidebarOpen ? "rounded-tl-3xl" : ""} relative`}
      >
        <div className="fixed top-3 left-3">
          <Button
            onClick={() => setSidebarOpen((prev) => !prev)}
            intent="ghost"
            size="square"
            rounded="circle"
            className="hover:bg-pink-50"
          >
            <ChevronRight
              className={`h-5 w-5 transition duration-200 ${sidebarOpen ? "rotate-180" : "rotate-0"}`}
            />
          </Button>
        </div>
        <Outlet />
        <div className="absolute bottom-0 flex justify-center inset-x-0 px-1 md:px-5 lg:px-10">
          <div className="w-full max-w-4xl bg-pink-50/90 backdrop-blur-3xl border border-b-0 border-pink-200 p-2 rounded-t-lg flex gap-2">
            <textarea
              value={message}
              onInput={(e) => setMessage(e.currentTarget.value)}
              className="resize-none w-full outline-none border-0 placeholder:font-display placeholder:text-pink-900/40 pl-2 pt-2"
              placeholder="Message Gemini 2.0 Flash"
            />
            <div className="text-center">
              <Button
                disabled={!message.trim().length}
                onClick={sendMessage}
                intent="primary"
                className="h-12 w-14"
              >
                <SendHorizonal className="text-pink-950 h-5 w-5" />
              </Button>
              <p className="text-xs font-display font-semibold text-black/40 flex items-center">
                <Command className="h-3 w-3 stroke-3 mr-0.5" />
                {" + "}Enter
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
