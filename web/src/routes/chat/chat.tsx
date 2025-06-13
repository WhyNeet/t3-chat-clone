import { useParams } from "react-router";
import { useEffect, useRef, useState } from "react";
import { fetchChatMessages } from "../../lib/api/messages";
import { isError } from "../../lib/api/error";
import { Role } from "../../lib/model/message";
import { Message } from "../../components/message";
import { useChatsStore } from "../../lib/state/chats";
import { Loader } from "../../components/ui/loader";
import { CircleAlert, Globe, Sparkle } from "lucide-react";

export function Chat() {
  const scrollWrapper = useRef<HTMLDivElement>(null);
  const scrollableContainer = useRef<HTMLDivElement>(null);
  const params = useParams();
  const chatId = params["chatId"] as string;
  const messages = useChatsStore((state) =>
    state.chats[chatId]?.state.status === "success"
      ? state.chats[chatId].messages
      : null,
  );
  const chatExists = useChatsStore(state => state.chats[chatId] !== undefined)
  const hasPendingMessage = useRef(false);
  const chatLoading = useChatsStore(
    (state) => state.chats[chatId]?.state.status === "loading",
  );
  const chat = useChatsStore(state => state.chats[chatId]);
  const chatsLoaded = useChatsStore(state => !state.isFetching);
  const setMessages = useChatsStore((state) => state.setChatMessages);
  const setChatState = useChatsStore((state) => state.setChatState);
  const pendingMessage = useChatsStore(
    (state) => state.pendingMessages[chatId],
  );
  const isSearching = useChatsStore(state => state.chats[chatId]?.searching);
  // const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    scrollWrapper.current.scrollTo({
      top: scrollableContainer.current.clientHeight,
      behavior: "instant",
    });
  }, [messages]);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    if (
      scrollableContainer.current.getBoundingClientRect().height -
      scrollWrapper.current.scrollTop - window.innerHeight + 300 >
      220 || hasPendingMessage.current === false && scrollableContainer.current.getBoundingClientRect().height -
      scrollWrapper.current.scrollTop - window.innerHeight + 300 >
      0
    )
      return;
    scrollWrapper.current.scrollTo({
      top: scrollableContainer.current.clientHeight,
      behavior: "instant",
    });
    hasPendingMessage.current = pendingMessage !== null;
  }, [pendingMessage]);

  useEffect(() => {
    if (!chatId || !chatExists) return;
    if (!chatLoading && !messages) {
      // setIsLoading(true);
      setChatState(chatId, { status: "loading" });
      fetchChatMessages(chatId, {
        start: 0,
        take: window.innerHeight / 100,
      }).then((result) => {
        if (isError(result)) {
          setChatState(chatId, { status: "error", error: result.error });
          setError(result.error);
          return;
        }
        setChatState(chatId, { status: "success" });
        setMessages(chatId, result);
        // setIsLoading(false);
      });
    } else {
      // setIsLoading(false);
    }
  }, [messages, chatId, setMessages, setChatState, chatLoading, chatExists]);

  return (
    <div
      className="w-full h-full overflow-y-scroll p-6 pb-32 pt-14 overscroll-contain"
      ref={scrollWrapper}
    >
      {chat ? <div className="absolute top-3 inset-x-0 flex items-center justify-center z-10 h-9">
        <div className="font-display px-4 py-1 text-sm font-medium rounded-full bg-pink-300/20 backdrop-blur-2xl text-pink-900 h-full flex items-center">{chat.chat.name}</div>
      </div> : null}
      {chatLoading ? (
        <div className="h-full w-full flex items-center justify-center gap-2">
          <Loader className="text-pink-600 h-5 w-5" />
          <p className="text-sm font-medium font-display">Loading chat...</p>
        </div>
      ) : chatsLoaded && !chatExists ? <div className="h-full w-full flex items-center justify-center gap-2 text-red-500">
        <CircleAlert className="h-5 w-5" />
        <p className="text-sm font-medium font-display">Chat does not exist.</p>
      </div> : null}
      {error ? `error: ${error}` : null}
      <div
        className="max-w-4xl flex flex-col-reverse mx-auto"
        ref={scrollableContainer}
      >
        {pendingMessage && (pendingMessage.content.length !== 0 || pendingMessage.reasoning && pendingMessage.reasoning.length !== 0) ? (
          <Message
            message={{
              chat_id: chatId,
              content: pendingMessage.content,
              model: pendingMessage.model,
              reasoning: pendingMessage.reasoning,
              id: "pending-message",
              role: Role.Assistant,
              timestamp: "",
            }}
          />
        ) : null}
        {isSearching ? <div className="p-4 flex items-center gap-4 text-pink-900 font-display animate-pulse mt-4">
          <Globe className="h-6 w-6" />
          <p>Searching...</p>
        </div> : (pendingMessage?.content.length === 0 && pendingMessage?.reasoning === null) ? <div className="p-4 flex items-center gap-4 text-pink-900 font-display animate-pulse mt-4">
          <Sparkle className="h-6 w-6" />
          <p>Waiting for inference...</p>
        </div> : null}
        {messages
          ? messages
            .filter(
              (message) =>
                message.content.length !== 0 ||
                (message.reasoning && message.reasoning.length !== 0),
            )
            .map((message) => <Message key={message.id} message={message} />)
          : null}
      </div>
    </div>
  );
}
