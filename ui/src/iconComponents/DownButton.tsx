import * as React from "react";

function SvgDownButton(props: React.SVGProps<SVGSVGElement>) {
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
        transform="rotate(-180 63.795 63.635)"
        strokeMiterlimit={10}
        fill="#999"
        stroke="#999"
      />
      <path
        stroke="#000"
        strokeMiterlimit={10}
        d="M61.94 112.27l49.32-53.26V14.73H15.74v44.28l46.2 53.26z"
      />
    </svg>
  );
}

export default SvgDownButton;
