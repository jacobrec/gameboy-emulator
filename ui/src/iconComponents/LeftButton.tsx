import * as React from "react";

function SvgLeftButton(props: React.SVGProps<SVGSVGElement>) {
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
        x={1.09}
        y={0.77}
        width={126}
        height={126}
        rx={7}
        transform="rotate(-90 63.66 63.93)"
        strokeMiterlimit={10}
        fill="#999"
        stroke="#999"
      />
      <path
        stroke="#000"
        strokeMiterlimit={10}
        d="M14.73 61.94l53.26 49.32h44.28V15.74H67.99l-53.26 46.2z"
      />
    </svg>
  );
}

export default SvgLeftButton;
