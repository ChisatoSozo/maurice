export type Emotion =
  | "suspicious"
  | "pleading"
  | "neutral"
  | "smiling"
  | "laughing"
  | "yes"
  | "no"
  | "angry"
  | "bored"
  | "headtilt";

export const Face = ({ emotion }: { emotion: Emotion }) => {
  return (
    <div id="face" className={emotion}>
      <div className="bg">
        <div className="eye">
          <div className="border">
            <div className="iris">
              <div className="glint"></div>
            </div>
          </div>

          <svg width="200px" height="200px" viewBox="0 0 200 200">
            <path d="M 0,0 V 100 q 100,-40 200,0 V 0 Z" className="upper lid" />
            <path
              d="M 0,100 q 100,-40 200,0 v 100 H 0 V 0 z"
              className="lower lid"
            />
          </svg>
        </div>
      </div>
      <div className="bg right">
        <div className="eye">
          <div className="border">
            <div className="iris">
              <div className="glint"></div>
            </div>
          </div>

          <svg width="200px" height="200px" viewBox="0 0 200 200">
            <path d="M 0,0 V 100 q 100,-40 200,0 V 0 Z" className="upper lid" />
            <path
              d="M 0,100 q 100,-40 200,0 v 100 H 0 V 0 z"
              className="lower lid"
            />
          </svg>
        </div>
      </div>
    </div>
  );
};
