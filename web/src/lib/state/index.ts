import { me } from "../api/auth";
import { chats } from "../api/chats";
import { is, subscribeToStream } from "../api/completions";
import { listUnsentFiles } from "../api/files";
import { listKeys } from "../api/keys";
import { listModels } from "../api/service";
import { useAuthStore } from "./auth";
import { useChatsStore } from "./chats";
import { useServiceStore } from "./service";

export async function init() {
  const { setModels, setKeys } = useServiceStore.getState();

  listKeys().then(keys => {
    setKeys(keys);
  });

  const models = await listModels();
  setModels(models);
  const { updateUser } = useAuthStore.getState();

  const { setInferenceError } = useServiceStore.getState();
  const {
    initializeChat,
    finishFetching,
    updatePendingMessage,
    clearPendingMessage,
    addChatMessages,
    initPendingMessage,
    finishWebSearch,
    updateChatName,
    updatePendingMessageMemory,
    setUploads
  } = useChatsStore.getState();
  listUnsentFiles(null).then(uploads => {
    setUploads("nochat", uploads);
  });
  useAuthStore.subscribe((store, prev) => {
    if (store.user && !prev.user) {
      chats({
        start: 0,
        take: Math.round((window.innerHeight * 1.5) / 50),
      }).then((chats) => {
        for (const chat of chats) {
          // listUnsentFiles(chat.id).then(uploads => {
          //   setUploads(chat.id, uploads);
          // });
          initializeChat(chat);
          const streamId = localStorage.getItem(`stream-${chat.id}`);
          if (streamId) {
            const isSearching =
              typeof localStorage.getItem(
                `streaming-message-${chat.id}-search`,
              ) === "string";
            const message =
              localStorage.getItem(`streaming-message-${chat.id}`) ?? "";
            const reasoning =
              localStorage.getItem(`streaming-message-reasoning-${chat.id}`) ??
              "";
            const model = localStorage.getItem(
              `streaming-message-${chat.id}-model`,
            );
            const memory = localStorage.getItem(`streaming-message-${chat.id}-memory`) ?? null;
            const provider_data = models.free.find(m => m.name === model) ?? models.paid.find(m => m.name === model)!;
            const provider = provider_data.base_url;
            initPendingMessage(chat.id, model ?? "AI", isSearching, memory);
            updatePendingMessage(chat.id, { content: message, reasoning, memory });
            subscribeToStream(
              streamId,
              (delta) => {
                updatePendingMessage(chat.id, {
                  reasoning: delta.reasoning,
                  content: delta.reasoning ? null : delta.content,
                  memory: null
                });
                localStorage.setItem(
                  `streaming-message-${chat.id}`,
                  (localStorage.getItem(`streaming-message-${chat.id}`) ?? "") +
                  delta.content,
                );
                localStorage.setItem(
                  `streaming-message-reasoning-${chat.id}`,
                  (localStorage.getItem(
                    `streaming-message-reasoning-${chat.id}`,
                  ) ?? "") + (delta.reasoning ?? ""),
                );
              },
              (control) => {
                if (is.webSearchPerformed(control.control)) {
                  localStorage.removeItem(
                    `streaming-message-${chat.id}-search`,
                  );
                  finishWebSearch(chat.id);
                } else if (is.chatNameUpdated(control.control)) {
                  updateChatName(chat.id, control.control.name);
                } else if (is.inferenceError(control.control)) {
                  if (provider === "https://https://openrouter.ai/api/v1/chat/completions") {
                    setInferenceError("openrouter", control.control.code);
                  }
                  localStorage.removeItem(`stream-${chat.id}`);
                  localStorage.removeItem(`streaming-message-${chat.id}`);
                  localStorage.removeItem(
                    `streaming-message-reasoning-${chat.id}`,
                  );
                  localStorage.removeItem(`streaming-message-${chat.id}-search`);
                  localStorage.removeItem(`streaming-message-${chat.id}-model`);
                  clearPendingMessage(chat.id);
                } else if (is.memoryAdded(control.control)) {
                  localStorage.setItem(
                    `streaming-message-${chat.id}-memory`,
                    control.control.memory,
                  );
                  updatePendingMessageMemory(chat.id, control.control.memory);
                }
              },
              (message) => {
                localStorage.removeItem(`stream-${chat.id}`);
                localStorage.removeItem(`streaming-message-${chat.id}`);
                localStorage.removeItem(
                  `streaming-message-reasoning-${chat.id}`,
                );
                localStorage.removeItem(`streaming-message-${chat.id}-search`);
                localStorage.removeItem(`streaming-message-${chat.id}-model`);
                clearPendingMessage(chat.id);
                addChatMessages(chat.id, [message]);
              },
            );
          }
        }
        finishFetching();
      });
    }
  });

  me().then((user) => {
    updateUser(user);
  });
}
