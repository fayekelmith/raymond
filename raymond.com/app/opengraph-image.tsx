import { ImageResponse } from "next/og";

export const runtime = "edge";

export const alt = "Raymond - Embedded Rust Journey";
export const size = {
  width: 1200,
  height: 630,
};
export const contentType = "image/png";

export default async function Image() {
  return new ImageResponse(
    (
      <div
        style={{
          fontSize: 128,
          background: "linear-gradient(to bottom right, #0a0a0a, #0d1117)",
          width: "100%",
          height: "100%",
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
          fontFamily: "monospace",
        }}
      >
        <div style={{ display: "flex", alignItems: "center" }}>
          <span
            style={{
              background: "linear-gradient(90deg, #00d4ff, #9f7aea)",
              backgroundClip: "text",
              color: "transparent",
              fontWeight: "bold",
            }}
          >
            Raymond
          </span>
        </div>
        <div
          style={{
            fontSize: 40,
            color: "#888",
            marginTop: 30,
            fontFamily: "sans-serif",
          }}
        >
          An Embedded Rust Journey
        </div>
      </div>
    ),
    {
      ...size,
    }
  );
}
