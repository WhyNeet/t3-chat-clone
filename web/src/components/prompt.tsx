import { useState } from "react";
import { Button } from "../components/ui/button";
import { Command, SendHorizonal } from "lucide-react";
import { useParams } from "react-router";
import { createMessage } from "../lib/api/messages";
import { subscribeToStream } from "../lib/api/completions";
import { createChat } from "../lib/api/chats";
import { useChatsStore } from "../lib/state/chats";
import { Loader } from "../components/ui/loader";
import { useNavigate } from "react-router";

export function Prompt() {
  const navigate = useNavigate();
  const [message, setMessage] = useState("");
  const updatePendingMessage = useChatsStore(
    (state) => state.updatePendingMessage,
  );
  const addMessages = useChatsStore((state) => state.addChatMessages);
  const addChat = useChatsStore((state) => state.initializeChat);
  const params = useParams();
  const chatId = params["chatId"];
  const pendingMessage = useChatsStore((store) =>
    chatId ? store.pendingMessages[chatId] : null,
  );
  const [isStreaming, setIsStreaming] = useState(!!pendingMessage);

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
    setMessage("");
    setIsStreaming(true);
    localStorage.setItem(`stream-${chatId}`, stream_id);
    subscribeToStream(
      stream_id,
      (delta) => {
        updatePendingMessage(chatId, delta.content);
        localStorage.setItem(`streaming-message-${chatId}`, (pendingMessage ?? "") + delta.content);
      },
      () => {
        localStorage.removeItem(`stream-${chatId}`);
        localStorage.removeItem(`streaming-message-${chatId}`);
        setIsStreaming(false);
      },
    );
  };

  return (
    <div className="w-full max-w-4xl bg-pink-50/90 backdrop-blur-3xl border border-b-0 border-pink-200 p-2 rounded-t-lg">
      <div className="flex gap-2">
        <textarea
          value={message}
          onInput={(e) => setMessage(e.currentTarget.value)}
          className="resize-none w-full outline-none border-0 placeholder:font-display placeholder:text-pink-900/40 pl-2 pt-2"
          placeholder="Message Gemini 2.0 Flash"
        />
        <div className="text-center">
          <Button
            disabled={!message.trim().length || isStreaming}
            onClick={sendMessage}
            intent="primary"
            className="h-12 w-14"
          >
            {isStreaming ? <Loader /> : <SendHorizonal className="h-5 w-5" />}
          </Button>
          <p
            className={`text-xs font-display font-semibold text-black/40 flex items-center select-none ${isStreaming ? "opacity-0" : "opacity-100"}`}
          >
            <Command className="h-3 w-3 stroke-3 mr-0.5" />
            {" + "}Enter
          </p>
        </div>
      </div>
    </div>
  );
}
