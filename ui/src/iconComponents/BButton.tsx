import * as React from "react";

function SvgBButton(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 125 125"
      width="1em"
      height="1em"
      {...props}
    >
      <circle
        cx={62.5}
        cy={62.5}
        r={62}
        fill="#91125c"
        stroke="#91125c"
        strokeMiterlimit={10}
        data-name="Layer 1"
      />
      <text
        transform="translate(45.27 94.56)"
        fontSize={84}
        fontFamily="FuturaBT-MediumCondensed,Futura Condensed BT"
        fontWeight={500}
      >
        {"B"}
      </text>
    </svg>
  );
}

export default SvgBButton;
