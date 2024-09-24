import React, { useState } from "react";
import { Box, Button, Input, Modal, Paper } from "@mui/material";
import { MauriceApi } from "../api";

export const CreateFileModal: React.FC<{
  open: boolean;
  onClose: () => void;
  path: string | null;
  onCreateFile: (path: string) => void;
}> = ({ open, onClose, path, onCreateFile }) => {
  const [newFileName, setNewFileName] = useState<string>("");

  const handleCreate = () => {
    if (newFileName && path) {
      const fullPath = path.replace("./files", "") + "/" + newFileName;
      MauriceApi.postApiCreateFile({
        path: fullPath,
      }).then(() => {
        onCreateFile(fullPath);
        onClose();
        setNewFileName("");
      });
    }
  };

  return (
    <Modal open={open} onClose={onClose}>
      <Box
        sx={{
          width: "100dvw",
          height: "100dvh",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Paper>
          <Input
            value={newFileName}
            onChange={(e) => setNewFileName(e.target.value)}
            placeholder="New file name"
          />
          <Button onClick={handleCreate}>Create</Button>
        </Paper>
      </Box>
    </Modal>
  );
};
