import { useParams } from "react-router";
import { useEffect, useRef, useState } from "react";
import { fetchChatMessages } from "../../lib/api/messages";
import { isError } from "../../lib/api/error";
import { Role } from "../../lib/model/message";
import { Message } from "../../components/message";
import { useChatsStore } from "../../lib/state/chats";
import { Loader } from "../../components/ui/loader";
import { AlertCircle, CircleAlert, Globe, Sparkle } from "lucide-react";
import { useServiceStore } from "../../lib/state/service";
import { Prompt } from "../../components/prompt";
import { useLocation } from "react-router";
import { listUnsentFiles } from "../../lib/api/files";

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
  const chatExists = useChatsStore(
    (state) => state.chats[chatId] !== undefined,
  );
  const hasPendingMessage = useRef(false);
  const chatLoading = useChatsStore(
    (state) => state.chats[chatId]?.state.status === "loading",
  );
  const chat = useChatsStore((state) => state.chats[chatId]);
  const chatsLoaded = useChatsStore((state) => !state.isFetching);
  const setMessages = useChatsStore((state) => state.setChatMessages);
  const prependMessages = useChatsStore((state) => state.prependChatMessages);
  const setChatState = useChatsStore((state) => state.setChatState);
  const pendingMessage = useChatsStore(
    (state) => state.pendingMessages[chatId],
  );
  const isSearching = useChatsStore((state) => state.chats[chatId]?.searching);
  // const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const isLastBatch = useRef(false);
  const [batchLoadError, setBatchLoadError] = useState<string | null>(null);
  const [batchLoading, setBatchLoading] = useState(false);
  const batchLoadingInternal = useRef(false);
  const inferenceError: number | null = useServiceStore(
    (state) => state.inferenceError["openrouter"] ?? null,
  );
  const location = useLocation();
  const setUploads = useChatsStore(state => state.setUploads);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    if (!chatLoading)
      scrollWrapper.current.scrollTo({
        top: scrollableContainer.current.clientHeight,
        behavior: "instant",
      });
  }, [chatLoading]);

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
    if (!scrollWrapper.current || !messages) return;
    const wrapper = scrollWrapper.current;
    const handler = () => {
      if (isLastBatch.current || batchLoading || batchLoadingInternal.current)
        return;
      if (wrapper.scrollTop < 100) {
        setBatchLoading(true);
        batchLoadingInternal.current = true;
        fetchChatMessages(chat.chat.id, {
          start: messages.length,
          take: window.innerHeight / 100,
        }).then((batch) => {
          if (isError(batch)) {
            setBatchLoadError(batch.error);
            return;
          }

          if (batch.length < Math.floor(window.innerHeight / 100))
            isLastBatch.current = true;
          prependMessages(chat.chat.id, batch);
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
    prependMessages,
    batchLoading,
    isLastBatch,
  ]);

  useEffect(() => {
    if (!scrollWrapper.current || !scrollableContainer.current) return;
    if (
      scrollableContainer.current!.getBoundingClientRect().height -
      scrollWrapper.current!.scrollTop -
      scrollWrapper.current!.getBoundingClientRect().height >
      -50
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
      listUnsentFiles(chatId).then(uploads => {
        setUploads(chatId, uploads);
      });
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
        scrollWrapper.current?.scrollTo({
          top: scrollableContainer.current?.clientHeight,
          behavior: "instant",
        });
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
      {chat ? (
        <div className="absolute top-3 inset-x-0 flex items-center justify-center z-10 h-9">
          <div className="font-display px-4 py-1 text-sm font-medium rounded-full bg-pink-300/20 backdrop-blur-2xl text-pink-900 h-full flex items-center">
            {chat.chat.name ?? "New Chat"}
          </div>
        </div>
      ) : null}
      {chatLoading ? (
        <div className="h-full w-full flex items-center justify-center gap-2">
          <Loader className="text-pink-600 h-5 w-5" />
          <p className="text-sm font-medium font-display">Loading chat...</p>
        </div>
      ) : chatsLoaded && !chatExists ? (
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
        {pendingMessage &&
          (pendingMessage.content.length !== 0 ||
            (pendingMessage.reasoning &&
              pendingMessage.reasoning.length !== 0)) ? (
          <Message
            message={{
              chat_id: chatId,
              content: [{ type: "Text", value: pendingMessage.content }],
              model: pendingMessage.model,
              reasoning: pendingMessage.reasoning,
              id: "pending-message",
              role: Role.Assistant,
              updated_memory: pendingMessage.memory,
              timestamp: "",
            }}
          />
        ) : null}
        {inferenceError ? (
          <div className="p-4 flex items-center gap-4 text-red-500 font-display mt-4">
            <AlertCircle className="h-6 w-6" />
            <p>Inference API error (code {inferenceError}).</p>
          </div>
        ) : null}
        {isSearching ? (
          <div className="p-4 flex items-center gap-4 text-pink-900 font-display animate-pulse mt-4">
            <Globe className="h-6 w-6" />
            <p>Searching...</p>
          </div>
        ) : pendingMessage?.content.length === 0 &&
          pendingMessage?.reasoning === null ? (
          <div className="p-4 flex items-center gap-4 text-pink-900 font-display animate-pulse mt-4">
            <Sparkle className="h-6 w-6" />
            <p>Waiting for inference...</p>
          </div>
        ) : null}
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
      <div className="absolute bottom-0 flex justify-center inset-x-0 px-1 md:px-5 lg:px-10">
        <Prompt />
      </div>
    </div>
  );
}
