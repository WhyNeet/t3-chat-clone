import { useState, useEffect } from "react";
import { Button } from "../components/ui/button";
import {
  Brain,
  ChevronDownIcon,
  Command,
  Globe,
  SendHorizonal,
} from "lucide-react";
import { useParams } from "react-router";
import { sendAndSubscribe } from "../lib/api/messages";
import { createChat } from "../lib/api/chats";
import { useChatsStore } from "../lib/state/chats";
import { Loader } from "../components/ui/loader";
import { useNavigate } from "react-router";
import { Popover, PopoverContent, PopoverTrigger } from "./ui/popover";
import { cn } from "./utils";
import { useServiceStore } from "../lib/state/service";
import type { Model } from "../lib/model/service";

export function Prompt() {
  const navigate = useNavigate();
  const [message, setMessage] = useState("");
  const addChat = useChatsStore((state) => state.initializeChat);
  const params = useParams();
  const chatId = params["chatId"];
  const [isRequesting, setIsRequesting] = useState(false);
  const models = useServiceStore((state) => state.models);
  const [isModelSelectorOpen, setIsModelSelectorOpen] = useState(false);
  const [isReasoning, setIsReasoning] = useState(false);
  const [selectedModel, setSelectedModel] = useState<Model>(null!);
  const [useSearch, setUseSearch] = useState(false);

  useEffect(() => {
    if (models) {
      const modelId = localStorage.getItem(`chat-model-${chatId}`);
      setSelectedModel(
        models.find(({ identifier }) => identifier === modelId) ?? models[0],
      );
    }
  }, [models, chatId]);

  useEffect(() => {
    if (!selectedModel || !chatId) return;
    localStorage.setItem(`chat-model-${chatId}`, selectedModel.identifier);
  }, [selectedModel, chatId]);

  const handleModelSelectorClick = (model: Model) => {
    setSelectedModel(model);
    setIsModelSelectorOpen(false);
  };

  const sendMessage = async () => {
    setIsRequesting(true);
    if (!chatId) {
      const chat = await createChat();
      addChat(chat, []);
      navigate(`/chat/${chat.id}`);
      sendAndSubscribe(
        chat.id,
        selectedModel,
        {
          message,
          model: selectedModel.identifier,
          reasoning: isReasoning ? "medium" : null,
          use_search: useSearch,
        },
        () => setIsRequesting(false),
      );
      setMessage("");
    } else {
      navigate(`/chat/${chatId}`);
      sendAndSubscribe(
        chatId,
        selectedModel,
        {
          message,
          model: selectedModel.identifier,
          reasoning: isReasoning ? "medium" : null,
          use_search: useSearch,
        },
        () => setIsRequesting(false),
      );
      setMessage("");
    }
  };

  return (
    <div className="w-full max-w-4xl bg-pink-50/90 backdrop-blur-3xl border border-b-0 border-pink-200 p-2 rounded-t-lg">
      <div className="flex gap-2">
        <textarea
          value={message}
          onInput={(e) => setMessage(e.currentTarget.value)}
          className="resize-none w-full outline-none border-0 placeholder:font-display placeholder:text-pink-900/40 pl-2 pt-2"
          placeholder={`Message ${selectedModel?.name}`}
        />
        <div className="text-center">
          <Button
            disabled={!message.trim().length || isRequesting}
            onClick={sendMessage}
            intent="primary"
            className="h-12 w-14"
          >
            {isRequesting ? <Loader /> : <SendHorizonal className="h-5 w-5" />}
          </Button>
          <p
            className={`text-xs font-display font-semibold text-black/40 flex items-center select-none ${isRequesting ? "opacity-0" : "opacity-100"}`}
          >
            <Command className="h-3 w-3 stroke-3 mr-0.5" />
            {" + "}Enter
          </p>
        </div>
      </div>
      <div className="flex gap-1 items-center h-7">
        {models && selectedModel ? (
          <>
            <Popover
              open={isModelSelectorOpen}
              onOpenChange={setIsModelSelectorOpen}
            >
              <PopoverTrigger asChild>
                <Button
                  intent="ghost"
                  size="small"
                  className="gap-1 text-pink-900"
                >
                  {selectedModel.name}
                  <ChevronDownIcon className="h-4 w-4" />
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-80 bg-white pt-4 pb-2">
                <h3 className="font-display font-semibold mb-2 px-4">Models</h3>
                <div className="flex flex-col w-full px-2">
                  {models.map((model) => (
                    <button
                      onClick={() => handleModelSelectorClick(model)}
                      className="py-3 px-4 hover:bg-pink-900/10 rounded-md transition text-left cursor-pointer text-sm font-display font-medium"
                      key={model.identifier}
                    >
                      {model.name}
                    </button>
                  ))}
                </div>
              </PopoverContent>
            </Popover>
            {selectedModel.is_reasoning ? (
              <Button
                onClick={() => setIsReasoning((prev) => !prev)}
                intent="ghost"
                size="small"
                className={cn(
                  "gap-1.5 border-pink-900/20 border rounded-full text-pink-900 transition",
                  isReasoning ? "hover:bg-pink-900/10 bg-pink-800/10" : "",
                )}
              >
                <Brain className="h-4 w-4" />
                Reasoning
              </Button>
            ) : null}
            <Button
              onClick={() => setUseSearch((prev) => !prev)}
              intent="ghost"
              size="small"
              className={cn(
                "gap-1.5 border-pink-900/20 border rounded-full text-pink-900 transition",
                useSearch ? "hover:bg-pink-900/10 bg-pink-800/10" : "",
              )}
            >
              <Globe className="h-4 w-4" />
              Web Search
            </Button>
          </>
        ) : null}
      </div>
    </div>
  );
}
