import React, { useCallback, useEffect, useMemo, useState } from "react";
import { Box, Container, IconButton, Input } from "@mui/material";
import { MauriceApi } from "../api";
import { FileItem } from "../components/FileItem";
import { Add, Search } from "@mui/icons-material";

export const DocumentsPage: React.FC = () => {
  const [files, setFiles] = useState<string[]>([]);
  const [search, setSearch] = useState("");

  const refreshFiles = useCallback(() => {
    MauriceApi.postApiListFiles({
      path: "/",
    }).then((files) => {
      setFiles(files.files);
      setSearch("");
    });
  }, []);
  useEffect(() => {
    refreshFiles();
  }, [refreshFiles]);

  const filteredFiles = useMemo(() => {
    return files.filter((file) =>
      file.toLowerCase().includes(search.toLowerCase())
    );
  }, [files, search]);

  const addFile = useCallback(() => {
    MauriceApi.postApiCreateFile({
      path: "/" + search,
    }).then(refreshFiles);
  }, [search, refreshFiles]);

  return (
    <Container>
      <Box width="100%">
        <Input
          endAdornment={<Search />}
          startAdornment={
            <IconButton onClick={addFile}>
              <Add />
            </IconButton>
          }
          fullWidth
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
      </Box>
      {filteredFiles.map((file) => (
        <FileItem key={file} path={file} updateFolder={refreshFiles} />
      ))}
    </Container>
  );
};
