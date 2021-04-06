import * as React from "react";

function SvgSelectButtonAngled(props: React.SVGProps<SVGSVGElement>) {
  return (
    <svg
      data-name="Layer 1"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 104 50"
      width="1em"
      height="1em"
      {...props}
    >
      <path
        d="M92.47 17L12.61 47.76a7.3 7.3 0 01-9.54-4.53l-.32-.91a8 8 0 014.57-10L87.19 1.66a7.29 7.29 0 019.54 4.52l.32.91A8 8 0 0192.47 17z"
        fill="gray"
        stroke="gray"
        strokeMiterlimit={10}
      />
      <text
        transform="matrix(.93 -.36 .36 .93 55.01 44.38)"
        fontSize={14}
        fontFamily="AvantGarde-Medium,ITC Avant Garde Gothic"
        fontWeight={500}
      >
        {"SELECT"}
      </text>
    </svg>
  );
}

export default SvgSelectButtonAngled;
