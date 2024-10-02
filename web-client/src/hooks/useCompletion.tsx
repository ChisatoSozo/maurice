import { Box, Checkbox, Input } from "@mui/material";
import { LoadCompletionModel } from "../components/LoadCompletionModel";
import { useCallback, useState, useEffect, useRef } from "react";
import { MauriceApi } from "../api";

export const useCompletion = () => {
  const [content, updateContent] = useState("");
  const [maxNewTokens, setMaxNewTokens] = useState(25);
  const loadingCompletion = useRef(false);
  const [completion, setCompletion] = useState<string | null>(null);
  const timeoutRef = useRef<number | null>(null);
  const [disableCompletion, setDisableCompletion] = useState(true);

  const acceptOrRejectCompletion = useCallback(() => {
    setCompletion(null);
  }, []);

  const complete = useCallback(async () => {
    if (loadingCompletion.current || disableCompletion) {
      return;
    }
    loadingCompletion.current = true;
    try {
      const response = await MauriceApi.postApiComplete({
        prompt: content,
        max_new_tokens: maxNewTokens,
      });
      const newText = response.value.replace(content, "");
      const newTextWithoutTrippleBackticks = newText.replace(/```/g, "");
      setCompletion(newTextWithoutTrippleBackticks);
    } catch (error) {
      console.error("Error during completion:", error);
    } finally {
      loadingCompletion.current = false;
    }
  }, [content, disableCompletion, maxNewTokens]);

  useEffect(() => {
    if (!content || disableCompletion) {
      return;
    }
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }

    timeoutRef.current = setTimeout(() => {
      complete();
    }, 500);

    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, [content, complete, disableCompletion]);

  return {
    component: (
      <Box display="flex" alignItems="center">
        <LoadCompletionModel />
        <Input
          type="number"
          value={maxNewTokens}
          style={{
            width: "45px",
            marginRight: "10px",
            marginLeft: "10px",
          }}
          onChange={(e) => setMaxNewTokens(parseInt(e.target.value))}
        />
        <Checkbox
          checked={!disableCompletion}
          onChange={() => setDisableCompletion(!disableCompletion)}
        />
      </Box>
    ),
    completion,
    acceptOrRejectCompletion,
    loadingCompletion,
    updateContent,
  };
};
