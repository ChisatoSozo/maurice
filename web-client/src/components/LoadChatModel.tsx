import { useCallback, useEffect, useState } from "react";
import { MauriceApi } from "../api";
import { Select } from "@mantine/core";
import { error } from "../utils/errorNotification";

export const LoadChatModel = () => {
  const [chatModels, setChatModels] = useState<string[]>([]);
  const [loadingChatModel, setLoadingChatModel] = useState(true);
  const [selectedChatModel, setSelectedChatModel] = useState<string | null>(
    null
  );

  useEffect(() => {
    MauriceApi.postApiListChatModels({ blank_arg: true })
      .then((response) => {
        setChatModels(response.value);
      })
      .catch((e) => {
        error(e);
      });
  }, []);

  useEffect(() => {
    if (chatModels.length > 0) {
      MauriceApi.postApiGetLoadedModelName({ blank_arg: true })
        .then((response) => {
          console.log(response);
          setSelectedChatModel(response.value);
        })
        .catch((e) => {
          error(e);
        })
        .finally(() => {
          setLoadingChatModel(false);
        });
    }
  }, [chatModels]);

  const handleChatModelChange = useCallback((value: string | null) => {
    if (!value) {
      return;
    }
    setLoadingChatModel(true);
    MauriceApi.postApiLoadModel({ model_name: value, cache_quant: 8 })
      .then(() => {
        setSelectedChatModel(value);
      })
      .catch((e) => {
        error(e);
      })
      .finally(() => {
        setLoadingChatModel(false);
      });
  }, []);

  return (
    <Select
      data={chatModels}
      placeholder="Select chat model"
      disabled={loadingChatModel}
      value={selectedChatModel}
      onChange={handleChatModelChange}
    />
  );
};
