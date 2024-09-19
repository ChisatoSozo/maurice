import {
  Accordion,
  AccordionDetails,
  AccordionSummary,
  Container,
  Typography,
} from "@mui/material";
import { ExpandMore } from "@mui/icons-material";
import { GetSpeakersReturn, MauriceApi } from "../api";
import { useEffect, useState } from "react";
import { Speaker } from "../components/Speaker";

export const AudioPage = () => {
  const [speakers, setSpeakers] = useState<GetSpeakersReturn["devices"]>([]);

  useEffect(() => {
    const fetchSpeakers = async () => {
      const speakers = await MauriceApi.postApiGetSpeakers();
      setSpeakers(speakers.devices);
    };

    fetchSpeakers();
  }, []);

  return (
    <Container>
      {speakers.map((speaker) => (
        <Accordion key={speaker.name}>
          <AccordionSummary expandIcon={<ExpandMore />} id={speaker.name}>
            <Typography>{speaker.name}</Typography>
          </AccordionSummary>
          <AccordionDetails>
            <Speaker name={speaker.name} />
          </AccordionDetails>
        </Accordion>
      ))}
    </Container>
  );
};
