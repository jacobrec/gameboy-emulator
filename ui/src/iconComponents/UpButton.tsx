import * as React from "react";

function SvgUpButton(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      data-name="Layer 1"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 127 127"
      width="1em"
      height="1em"
      {...props}
    >
      <rect
        x={0.5}
        y={0.5}
        width={126}
        height={126}
        rx={7}
        strokeMiterlimit={10}
        fill="#999"
        stroke="#999"
      />
      <path
        stroke="#000"
        strokeMiterlimit={10}
        d="M65.06 14.73L15.74 67.99v44.28h95.52V67.99l-46.2-53.26z"
      />
    </svg>
  );
}

export default SvgUpButton;
