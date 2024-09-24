import React, { useState } from "react";
import { Box, Button, Input, Modal, Paper } from "@mui/material";
import { MauriceApi } from "../api";

export const CreateDirectoryModal: React.FC<{
  open: boolean;
  onClose: () => void;
  path: string | null;
  onCreateDirectory: (path: string) => void;
}> = ({ open, onClose, path, onCreateDirectory }) => {
  const [newDirectoryName, setNewDirectoryName] = useState<string>("");

  const handleCreate = () => {
    if (newDirectoryName && path) {
      const fullPath = path.replace("./files", "") + "/" + newDirectoryName;
      MauriceApi.postApiCreateDirectory({
        path: fullPath,
      }).then(() => {
        onCreateDirectory(fullPath);
        onClose();
        setNewDirectoryName("");
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
            value={newDirectoryName}
            onChange={(e) => setNewDirectoryName(e.target.value)}
            placeholder="New directory name"
          />
          <Button onClick={handleCreate}>Create</Button>
        </Paper>
      </Box>
    </Modal>
  );
};
