import { useCallback, useEffect, useState } from "react";
import { MauriceApi } from "../api";

import { MenuItem, Select } from "@mui/material";

export const LoadCompletionModel = () => {
  const [completionModels, setCompletionModels] = useState<string[]>([]);
  const [loadingCompletionModel, setLoadingCompletionModel] = useState(true);
  const [selectedCompletionModel, setSelectedCompletionModel] = useState<
    string | null
  >(null);

  useEffect(() => {
    MauriceApi.postApiListCompletionModels({ blank_arg: true })
      .then((response) => {
        setCompletionModels(response.value);
      })
      .catch((e) => {
        console.log(e);
      });
  }, []);

  useEffect(() => {
    if (completionModels.length > 0) {
      MauriceApi.postApiGetLoadedCompletionModelName({ blank_arg: true })
        .then((response) => {
          setSelectedCompletionModel(response.value || null);
        })
        .catch((e) => {
          console.log(e);
        })
        .finally(() => {
          setLoadingCompletionModel(false);
        });
    }
  }, [completionModels]);

  const handleCompletionModelChange = useCallback((value: string | null) => {
    if (!value) {
      return;
    }
    setLoadingCompletionModel(true);
    MauriceApi.postApiLoadCompletionModel({ model_name: value, cache_quant: 4 })
      .then(() => {
        setSelectedCompletionModel(value);
      })
      .catch((e) => {
        console.log(e);
      })
      .finally(() => {
        setLoadingCompletionModel(false);
      });
  }, []);

  return (
    <Select
      style={{
        width: "100px",
        height: "2rem",
      }}
      placeholder="Select completion model"
      disabled={loadingCompletionModel}
      value={selectedCompletionModel || ""}
      onChange={(e) => handleCompletionModelChange(e.target.value)}
    >
      {completionModels.map((model) => (
        <MenuItem key={model} value={model}>
          {model}
        </MenuItem>
      ))}
    </Select>
  );
};
