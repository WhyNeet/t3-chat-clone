import { useParams, useSearchParams } from "react-router";
import { lazy, useEffect, useRef, useState } from "react";
import { fetchChatMessages } from "../../lib/api/messages";
import { isError } from "../../lib/api/error";
import { type ChatMessage } from "../../lib/model/message";
import { Loader } from "../../components/ui/loader";
import { AlertCircle, CircleAlert } from "lucide-react";
import type { Chat } from "../../lib/model/chat";
import { getChatShate } from "../../lib/api/chats";

const Message = lazy(() => import("../../components/message"));

export function SharedChat() {
  const scrollWrapper = useRef<HTMLDivElement>(null);
  const scrollableContainer = useRef<HTMLDivElement>(null);
  const params = useParams();
  const chatId = params["chatId"] as string;
  const [query] = useSearchParams();
  const shareId = query.get("id");
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [chatExists, setChatExists] = useState(true);
  const [chat, setChat] = useState<Chat | null>(null);
  const [error, setError] = useState<string | null>(null);
  const isLastBatch = useRef(false);
  const [batchLoadError, setBatchLoadError] = useState<string | null>(null);
  const [batchLoading, setBatchLoading] = useState(false);
  const batchLoadingInternal = useRef(false);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    if (!isLoading)
      scrollWrapper.current.scrollTo({
        top: scrollableContainer.current.clientHeight,
        behavior: "instant",
      });
  }, [isLoading]);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    scrollWrapper.current.scrollTo({
      top: scrollableContainer.current.clientHeight,
      behavior: "instant",
    });
  }, [location]);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    if (
      scrollableContainer.current!.getBoundingClientRect().height -
      scrollWrapper.current!.scrollTop -
      scrollWrapper.current!.getBoundingClientRect().height <
      0
    )
      scrollWrapper.current.scrollTo({
        top: scrollableContainer.current.clientHeight,
        behavior: "instant",
      });
  }, [messages]);

  useEffect(() => {
    if (!scrollWrapper.current || !messages || !chat || !shareId) return;
    const wrapper = scrollWrapper.current;
    const handler = () => {
      if (isLastBatch.current || batchLoading || batchLoadingInternal.current)
        return;
      if (wrapper.scrollTop < 100) {
        setBatchLoading(true);
        batchLoadingInternal.current = true;
        fetchChatMessages(chat.id, {
          start: messages.length,
          take: window.innerHeight / 100,
        }, shareId).then((batch) => {
          if (isError(batch)) {
            setBatchLoadError(batch.error);
            return;
          }

          if (batch.length < Math.floor(window.innerHeight / 100))
            isLastBatch.current = true;
          setMessages(prev => [...prev, ...batch]);
          setBatchLoading(false);
          batchLoadingInternal.current = false;
        });
      }
    };
    wrapper.addEventListener("scroll", handler);

    return () => wrapper.removeEventListener("scroll", handler);
  }, [
    scrollWrapper,
    messages,
    chat,
    batchLoading,
    isLastBatch,
  ]);

  useEffect(() => {
    if (isLoading || chat || !shareId) return;
    setIsLoading(true);
    getChatShate(chatId, shareId).then(chat => {
      if (!chat) {
        setChatExists(false);
        setIsLoading(false);
        return;
      }
      setChat(chat);
    });
  }, [chatId]);

  useEffect(() => {
    if (!chat || !shareId) return;

    fetchChatMessages(chat.id, {
      start: 0,
      take: window.innerHeight / 100,
    }, shareId).then((result) => {
      if (isError(result)) {
        setError(result.error);
        return;
      }
      setMessages(result);
      scrollWrapper.current?.scrollTo({
        top: scrollableContainer.current?.clientHeight,
        behavior: "instant",
      });
      setIsLoading(false);
    });
  }, [chat])

  return (
    <div
      className="w-full h-full overflow-y-scroll p-6 pb-10 pt-14 overscroll-contain"
      ref={scrollWrapper}
    >
      {chat ? (
        <div className="absolute top-3 inset-x-0 flex items-center justify-center z-10 h-9">
          <div className="font-display px-4 py-1 text-sm font-medium rounded-full bg-pink-300/20 backdrop-blur-2xl text-pink-900 h-full flex items-center">
            {chat.name ?? "New Chat"}
          </div>
        </div>
      ) : null}
      {isLoading ? (
        <div className="h-full w-full flex items-center justify-center gap-2">
          <Loader className="text-pink-600 h-5 w-5" />
          <p className="text-sm font-medium font-display">Loading chat...</p>
        </div>
      ) : !chatExists ? (
        <div className="h-full w-full flex items-center justify-center gap-2 text-red-500">
          <CircleAlert className="h-5 w-5" />
          <p className="text-sm font-medium font-display">
            Chat does not exist.
          </p>
        </div>
      ) : null}
      {error ? `error: ${error}` : null}
      <div
        className="max-w-4xl flex flex-col-reverse mx-auto"
        ref={scrollableContainer}
      >
        {messages
          ? messages
            .filter(
              (message) =>
                message.content.length !== 0 ||
                (message.reasoning && message.reasoning.length !== 0),
            )
            .map((message) => <Message key={message.id} message={message} />)
          : null}
        {batchLoading ? (
          <div className="p-4 flex items-center gap-4 text-pink-900 font-display animate-pulse mt-4">
            <Loader className="h-6 w-6" />
            <p>Loading messages...</p>
          </div>
        ) : null}
        {batchLoadError ? (
          <div className="p-4 flex items-center gap-4 text-red-500 font-display mt-4">
            <AlertCircle className="h-6 w-6" />
            <p>Failed to load messages: {batchLoadError}.</p>
          </div>
        ) : null}
      </div>
    </div>
  );
}
