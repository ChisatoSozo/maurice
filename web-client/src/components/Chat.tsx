import { Box, TextInput } from "@mantine/core";
import { useCallback, useState, useRef, useEffect } from "react";
import { MauriceApi } from "../api";
import { Emotion } from "./Face";

type Message = {
  text: string;
  isUser: boolean;
};

export const Chat = ({
  setEmotion,
}: {
  setEmotion: (emotion: Emotion) => void;
}) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [sending, setSending] = useState(false);
  const [inputValue, setInputValue] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const sendMessage = useCallback((text: string) => {
    if (text.trim() === "") return;

    setMessages((prev) => [...prev, { text, isUser: true }]);
    setSending(true);
    setInputValue("");

    MauriceApi.postApiSendChat({ message: text, username: "user" })
      .then((response) => {
        let text = response.value;
        //text contains <emotion>emotion</emotion>, remove it and set the emotion
        const emotionMatch = text.match(/<emotion>(\w+)<\/emotion>/);
        if (emotionMatch) {
          text = text.replace(emotionMatch[0], "");
          setEmotion(emotionMatch[1] as Emotion);
        }
        setMessages((prev) => [...prev, { text, isUser: false }]);
      })
      .catch((e) => {
        console.error(e);
        setMessages((prev) => {
          const newMessages = [...prev];
          newMessages.pop();
          return newMessages;
        });
      })
      .finally(() => {
        setSending(false);
      });
  }, []);

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !sending) {
      e.preventDefault();
      sendMessage(inputValue);
    }
  };

  // Use useEffect to focus the input field after sending a message
  useEffect(() => {
    if (!sending && inputRef.current) {
      inputRef.current.focus();
    }
  }, [sending]);

  return (
    <Box
      w="100%"
      h="100%"
      p="lg"
      mih={0}
      style={{ display: "flex", flexDirection: "column", flexGrow: 1 }}
    >
      <Box
        style={{
          flex: 1,
          overflowY: "auto",
          display: "flex",
          minHeight: 0,
          flexDirection: "column-reverse",
        }}
      >
        {[...messages].reverse().map((message, index) => (
          <Box
            key={index}
            style={{
              alignSelf: message.isUser ? "flex-end" : "flex-start",
              maxWidth: "50%",
              padding: "10px",
              margin: "10px",
              borderRadius: "10px",
              backgroundColor: message.isUser ? "#f0f0f0" : "#f0f0f0",
            }}
          >
            {message.text}
          </Box>
        ))}
      </Box>
      <Box
        style={{
          display: "flex",
          flexDirection: "row",
          gap: "10px",
          alignItems: "center",
        }}
      >
        <TextInput
          placeholder="Type a message..."
          style={{ flex: 1 }}
          value={inputValue}
          onChange={(e) => setInputValue(e.currentTarget.value)}
          onKeyDown={handleKeyDown}
          disabled={sending}
          tabIndex={0}
          ref={inputRef}
        />
      </Box>
    </Box>
  );
};
