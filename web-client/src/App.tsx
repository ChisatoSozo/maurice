import { Box } from "@mantine/core";
import { LoadChatModel } from "./components/LoadChatModel";
import { Chat } from "./components/Chat";
import { Emotion, Face } from "./components/Face";
import { useState } from "react";

export const App = () => {
  const [emotion, setEmotion] = useState<Emotion>("neutral");
  return (
    <Box
      h="100dvh"
      w="100vw"
      style={{
        display: "flex",
        flexDirection: "column",
      }}
    >
      <Box
        pos="absolute"
        w="100vw"
        top={50}
        left={-25}
        display="flex"
        style={{
          justifyContent: "center",
        }}
      >
        <Face emotion={emotion} />
      </Box>

      <Box w="100%">
        <LoadChatModel />
      </Box>
      <Box flex={1} w="100%" mih={0}>
        <Chat setEmotion={setEmotion} />
      </Box>
    </Box>
  );
};
