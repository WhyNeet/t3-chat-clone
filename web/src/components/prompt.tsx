import { useState, useEffect, useRef, type ChangeEvent } from "react";
import { Button } from "../components/ui/button";
import {
  Brain,
  ChevronDownIcon,
  CircleAlert,
  Command,
  Database,
  FileText,
  Globe,
  Key,
  Paperclip,
  SendHorizonal,
  XIcon,
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
import { useAuthStore } from "../lib/state/auth";
import { toast } from "sonner";
import { deleteFile, getFileUri, uploadFile } from "../lib/api/files";
import { Tooltip, TooltipContent, TooltipTrigger } from "./ui/tooltip";

export default function Prompt() {
  const navigate = useNavigate();
  const [message, setMessage] = useState("");
  const addChat = useChatsStore((state) => state.initializeChat);
  const params = useParams();
  const isAuthorized = useAuthStore((state) => state.user !== null);
  const chatId = params["chatId"];
  const [isRequesting, setIsRequesting] = useState(false);
  const models = useServiceStore((state) => state.models);
  const [isModelSelectorOpen, setIsModelSelectorOpen] = useState(false);
  const [isReasoning, setIsReasoning] = useState(false);
  const [selectedModel, setSelectedModel] = useState<Model>(null!);
  const [useSearch, setUseSearch] = useState(false);
  const uploads = useChatsStore((state) =>
    chatId ? state.uploads[chatId] : state.uploads["nochat"],
  );
  const addUpload = useChatsStore((state) => state.addUpload);
  const removeUpload = useChatsStore((state) => state.removeUpload);
  const clearUploads = useChatsStore(state => state.clearUploads);
  const uploadInputRef = useRef<HTMLInputElement>(null);
  const [isUploading, setIsUploading] = useState(false);
  const keys = useServiceStore(state => state.keys);
  const [useMemories, setUseMemories] = useState(localStorage.getItem("use-memories") !== null);

  useEffect(() => {
    if (useMemories) localStorage.setItem("use-memories", "");
    else localStorage.removeItem("use-memories");
  }, [useMemories])

  const handleUploadClick = () => {
    if (!uploadInputRef.current) return;
    uploadInputRef.current.click();
  };

  useEffect(() => {
    if (models) {
      const modelId = localStorage.getItem(`chat-model-${chatId}`);
      setSelectedModel(
        (models.free.find(({ identifier }) => identifier === modelId) ?? models.paid.find(({ identifier }) => identifier === modelId)) ?? models.free[0],
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

  const handleFileInputChange = async (e: ChangeEvent<HTMLInputElement>) => {
    const file = e.currentTarget.files![0];
    if (file.size > 1000000) {
      toast(<div className="flex items-center gap-2">
        <CircleAlert className="h-5 w-5" />
        File is too large (max 1mb).
      </div>, { className: "bg-red-50! border border-red-500! rounded-lg text-red-500! font-display font-medium" });
      return;
    }

    const formData = new FormData();
    formData.append("file", file);

    setIsUploading(true);
    const upload = await uploadFile(chatId ?? null, formData);
    addUpload("nochat", upload);
    setIsUploading(false);
  }

  const handleFileRemove = async (chatId: string | null, fileId: string) => {
    await deleteFile(chatId, fileId);
    removeUpload(chatId ?? "nochat", fileId);
  }

  const sendMessage = async () => {
    setIsRequesting(true);
    if (!chatId) {
      const chat = await createChat();
      addChat(chat, []);
      localStorage.setItem(`chat-model-${chatId}`, selectedModel.identifier);
      navigate(`/chat/${chat.id}`);
      sendAndSubscribe(
        chat.id,
        selectedModel,
        {
          message,
          model: selectedModel.identifier,
          reasoning: isReasoning ? "medium" : null,
          use_search: useSearch,
          use_memories: useMemories
        },
        () => setIsRequesting(false),
      );
      clearUploads("nochat");
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
          use_memories: useMemories
        },
        () => setIsRequesting(false),
      );
      setMessage("");
      clearUploads(chatId);
    }
  };

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.metaKey && e.key === "Enter") {
        sendMessage();
      }
    };
    window.addEventListener("keydown", handler);

    return () => window.removeEventListener("keydown", handler);
  });

  if (!isAuthorized) return <></>;

  return (
    <div className="w-full max-w-4xl bg-pink-50/90 backdrop-blur-3xl border border-b-0 border-pink-200 p-2 rounded-t-lg">
      <div className="max-w-full overflow-x-scroll scrollbar-none">
        {uploads && uploads.length > 0 ? <div className="flex gap-2 mb-2">
          {uploads.map(upload => <div key={upload.id} className="rounded-lg h-20 min-w-20 group relative overflow-hidden">
            <div className="h-20 min-w-20 aspect-square overflow-hidden rounded-md transition group-hover:brightness-50 flex items-center justify-center bg-white">
              {upload.content_type.startsWith("image/") ? <img src={getFileUri(chatId ?? null, upload.id)} className="h-20 min-w-fit" />
                : <FileText className="h-7 w-7 text-pink-900" />}</div>
            <button onClick={() => handleFileRemove(upload.chat_id, upload.id)}
              className="absolute top-2 left-2 text-white cursor-pointer transition opacity-0 group-hover:opacity-100"
            >
              <XIcon className="h-5 w-5" />
            </button>
          </div>)}
        </div> : null}
      </div>
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
                  className="gap-1 rounded-full bg-pink-800/10 text-pink-900 pl-2.5 border border-transparent hover:bg-pink-900/20"
                >
                  {selectedModel.name}
                  <ChevronDownIcon className="h-4 w-4" />
                </Button>
              </PopoverTrigger>
              <PopoverContent className="w-80 bg-white pt-4">
                <h3 className="font-display font-semibold mb-2 px-4">Models</h3>
                <div className="h-[50vh] overflow-y-scroll scrollbar-none">
                  <div className="flex flex-col w-full px-2 pb-2">
                    {models.free.map((model) => (
                      <button
                        onClick={() => handleModelSelectorClick(model)}
                        className="py-3 px-4 hover:bg-pink-900/10 rounded-md transition text-left cursor-pointer whitespace-nowrap"
                        key={model.identifier}
                      >
                        <p className="text-xs font-display font-medium text-pink-900/60">
                          {model.author}
                        </p>
                        <div className="text-sm font-display font-medium">
                          {model.name}
                        </div>
                      </button>
                    ))}
                    {models.paid.map((model) => (
                      <button
                        onClick={() => handleModelSelectorClick(model)}
                        disabled={keys !== null && keys.length === 0}
                        className="py-3 px-4 pr-2 hover:bg-pink-900/10 rounded-md transition cursor-pointer disabled:opacity-50 whitespace-nowrap text-left"
                        key={model.identifier}
                      >
                        <div className="text-xs font-display font-medium text-pink-900/60 flex items-center gap-1 h-4">
                          {model.author}

                          <div className="h-0.5 w-0.5 rounded-full bg-pink-900/60"></div>
                          <Tooltip>
                            <TooltipTrigger asChild>
                              <Key className="h-3 w-3 stroke-3 text-pink-900/60" />
                            </TooltipTrigger>
                            <TooltipContent>
                              Bring Your Own Key
                            </TooltipContent>
                          </Tooltip>
                        </div>
                        <div className="text-sm font-display font-medium flex items-center gap-2">
                          {model.name}
                        </div>
                      </button>
                    ))}
                  </div>
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
            <Button
              onClick={() => setUseMemories((prev) => !prev)}
              intent="ghost"
              size="small"
              className={cn(
                "gap-1.5 border-pink-900/20 border rounded-full text-pink-900 transition",
                useMemories ? "hover:bg-pink-900/10 bg-pink-800/10" : "",
              )}
            >
              <Database className="h-4 w-4" />
              Memories
            </Button>
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  onClick={() => handleUploadClick()}
                  intent="ghost"
                  size="square"
                  disabled={isUploading}
                  rounded="circle"
                  className={cn(
                    "gap-1.5 border-pink-900/20 border text-pink-900 transition h-[30px] w-[30px] disabled:opacity-80 disabled:hover:bg-transparent disabled:cursor-not-allowed",
                  )}
                >
                  {isUploading ? <Loader className="h-4 w-4" /> : <Paperclip className="h-4 w-4" />}
                </Button>
              </TooltipTrigger>
              <TooltipContent>
                Supports PNG, JPEG and PDF up to 1MB
              </TooltipContent>
            </Tooltip>
          </>
        ) : null}
      </div>
      <input onChange={handleFileInputChange} ref={uploadInputRef} className="hidden" type="file" multiple={false} accept="image/png, image/jpeg, application/pdf" />
    </div>
  );
}
