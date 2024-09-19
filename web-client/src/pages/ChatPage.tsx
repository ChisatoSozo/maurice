// import { LoadChatModel } from "../components/LoadChatModel";
// import { Chat } from "../components/Chat";
// import { Emotion, Face } from "../components/Face";
// import { useState } from "react";
// import { Box } from "@mui/material";

// export const ChatPage = () => {
//   const [emotion, setEmotion] = useState<Emotion>("neutral");
//   return (
//     <Box
//       style={{
//         height: "100%",
//         width: "100%",
//         display: "flex",
//         flexDirection: "column",
//       }}
//     >
//       <Box
//         pos="absolute"
//         w="100%"
//         top={50}
//         left={-25}
//         display="flex"
//         style={{
//           justifyContent: "center",
//         }}
//       >
//         <Face emotion={emotion} />
//       </Box>

//       <Box w="100%">
//         <LoadChatModel />
//       </Box>
//       <Box flex={1} w="100%" mih={0}>
//         <Chat setEmotion={setEmotion} />
//       </Box>
//     </Box>
//   );
// };
