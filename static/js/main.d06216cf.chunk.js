(this["webpackJsonpgameboy-ui"]=this["webpackJsonpgameboy-ui"]||[]).push([[1],{83:function(t,e,n){},84:function(t,e,n){},96:function(t,e,n){"use strict";n.r(e);var a,c=n(1),o=n(0),r=n.n(o),s=n(10),i=n.n(s),l=(n(83),n(4)),j=n(13),u=n(14),b=(n(84),n(12)),d=n.n(b),x=n(21),O=n(61),h=n(62),m=n(43),f=n.n(m);!function(t){t[t.Start=0]="Start",t[t.Select=1]="Select",t[t.DUp=2]="DUp",t[t.DDown=3]="DDown",t[t.DLeft=4]="DLeft",t[t.DRight=5]="DRight",t[t.A=6]="A",t[t.B=7]="B"}(a||(a={}));var p=function(){function t(){var e=this;Object(O.a)(this,t),this.wasm=null,Object(x.a)(d.a.mark((function t(){var a;return d.a.wrap((function(t){for(;;)switch(t.prev=t.next){case 0:return t.next=2,n.e(0).then(n.bind(null,151));case 2:return t.next=4,Promise.all([n.e(0),n.e(4)]).then(n.bind(null,153));case 4:a=t.sent,e.wasm=a;case 6:case"end":return t.stop()}}),t)})))()}return Object(h.a)(t,[{key:"load_rom",value:function(t){var e,n=this,a=window;(a.lf=f.a,a.has_loaded)||(null===(e=this.wasm)||void 0===e||e.init(t),a.has_loaded=!0);a.button_down=function(t){return n.button_down(t)},a.button_up=function(t){return n.button_up(t)}}},{key:"button_down",value:function(t){var e;return console.log("Sending Button press: ",t),null===(e=this.wasm)||void 0===e?void 0:e.button_down(t)}},{key:"button_up",value:function(t){var e;return null===(e=this.wasm)||void 0===e?void 0:e.button_up(t)}},{key:"make_save_state",value:function(){var t,e=null===(t=this.wasm)||void 0===t?void 0:t.save_state(),n={date:Date.now(),data:e};f.a.getItem("saves").then((function(t){null===t&&(t=[]),t.push(n),f.a.setItem("saves",t)}))}},{key:"load_save_state",value:function(){var t=this;f.a.getItem("saves").then((function(e){var n;if(null!==e){var a=e[e.length-1];console.log("Loading save ",a),null===(n=t.wasm)||void 0===n||n.load_state(a.data)}else console.log("No Saves to load from")}))}},{key:"update",value:function(){var t;return null===(t=this.wasm)||void 0===t?void 0:t.update(35128)}}]),t}(),g=function(t){var e=window,n=Object(o.useState)(!1),a=Object(j.a)(n,2),r=a[0],s=a[1],i=Object(o.useState)(new p),l=Object(j.a)(i,2),u=l[0],b=(l[1],t.id),d=t.rom;return Object(o.useEffect)((function(){var t=document.getElementById(b),e=null;null!==t&&(e=t.getContext("2d"));var n=new ImageData(160,144),a=0;return r&&(a=requestAnimationFrame((function t(c){c;var o=u.update();n.data.set(new Uint8ClampedArray(o.buffer)),e.putImageData(n,0,0),a=requestAnimationFrame(t)}))),function(){cancelAnimationFrame(a)}})),r?(console.log(e.rom),u.load_rom(e.rom),e.emu=u):d.constructor===File&&d.arrayBuffer().then((function(t){e.rom=new Uint8Array(t),s(!0)})),Object(c.jsx)("canvas",{id:b,width:160,height:144})},w=n(57),v=n(24),y=n.n(v),k=n(142),S=n(143),B=n(53),C=n(140),N=n(69),L=n.n(N),D=n(132),M=n(149),_=n(146),P=n(144),F=n(138),I=n(145),A=n(147),E=n(139),z=n(134),G=n(136),T=n(137),R=n(141),U=n(64),J=n.n(U),W=n(65),K=n.n(W),q=n(63),H=n.n(q),V=n(66),Q=n.n(V),X=n(67),Y=n.n(X),Z=n(148),$=n(68),tt=n.n($);var et=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({"data-name":"Layer 1",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 127 127",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("rect",{x:1.09,y:.77,width:126,height:126,rx:7,transform:"rotate(-180 63.795 63.635)",strokeMiterlimit:10,fill:"#999",stroke:"#999"}),Object(c.jsx)("path",{stroke:"#000",strokeMiterlimit:10,d:"M61.94 112.27l49.32-53.26V14.73H15.74v44.28l46.2 53.26z"})]}))};var nt=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({"data-name":"Layer 1",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 127 127",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("rect",{x:.5,y:.5,width:126,height:126,rx:7,strokeMiterlimit:10,fill:"#999",stroke:"#999"}),Object(c.jsx)("path",{stroke:"#000",strokeMiterlimit:10,d:"M65.06 14.73L15.74 67.99v44.28h95.52V67.99l-46.2-53.26z"})]}))};var at=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({"data-name":"Layer 1",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 127 127",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("rect",{x:1.09,y:.77,width:126,height:126,rx:7,transform:"rotate(-90 63.66 63.93)",strokeMiterlimit:10,fill:"#999",stroke:"#999"}),Object(c.jsx)("path",{stroke:"#000",strokeMiterlimit:10,d:"M14.73 61.94l53.26 49.32h44.28V15.74H67.99l-53.26 46.2z"})]}))};var ct=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({"data-name":"Layer 1",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 127 127",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("rect",{x:1.09,y:.77,width:126,height:126,rx:7,transform:"rotate(90 63.93 63.34)",strokeMiterlimit:10,fill:"#999",stroke:"#999"}),Object(c.jsx)("path",{stroke:"#000",strokeMiterlimit:10,d:"M112.27 65.06L59.01 15.74H14.73v95.52h44.28l53.26-46.2z"})]}))};var ot=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({"data-name":"Layer 1",xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 104 50",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("path",{d:"M92.47 17L12.61 47.76a7.3 7.3 0 01-9.54-4.53l-.32-.91a8 8 0 014.57-10L87.19 1.66a7.29 7.29 0 019.54 4.52l.32.91A8 8 0 0192.47 17z",fill:"gray",stroke:"gray",strokeMiterlimit:10}),Object(c.jsx)("text",{transform:"matrix(.93 -.36 .36 .93 55.01 44.38)",fontSize:14,fontFamily:"AvantGarde-Medium,ITC Avant Garde Gothic",fontWeight:500,children:"SELECT"})]}))};var rt=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 96.14 48.08",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("path",{d:"M90.64 16.33L10.78 47.09a7.3 7.3 0 01-9.54-4.53l-.32-.91a8 8 0 014.57-10L85.36.99a7.29 7.29 0 019.54 4.52l.32.91a8 8 0 01-4.58 9.91z",fill:"gray",stroke:"gray",strokeMiterlimit:10,"data-name":"Layer 1"}),Object(c.jsxs)("text",{transform:"matrix(.93 -.36 .36 .93 57.67 41.69)",fontSize:14,fontFamily:"AvantGarde-Medium,ITC Avant Garde Gothic",fontWeight:500,children:["S",Object(c.jsx)("tspan",{x:6.99,y:0,letterSpacing:"-.02em",children:"T"}),Object(c.jsx)("tspan",{x:12.33,y:0,children:"A"}),Object(c.jsx)("tspan",{x:21.91,y:0,letterSpacing:".01em",children:"R"}),Object(c.jsx)("tspan",{x:30.44,y:0,letterSpacing:0,children:"T"})]})]}))};var st=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 125 125",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("circle",{cx:62.5,cy:62.5,r:62,fill:"#91125c",stroke:"#91125c",strokeMiterlimit:10,"data-name":"Layer 1"}),Object(c.jsx)("text",{transform:"translate(44.02 90.56)",fontSize:84,fontFamily:"FuturaBT-MediumCondensed,Futura Condensed BT",fontWeight:500,children:"A"})]}))};var it=function(t){return Object(c.jsxs)("svg",Object(l.a)(Object(l.a)({xmlns:"http://www.w3.org/2000/svg",viewBox:"0 0 125 125",width:"1em",height:"1em"},t),{},{children:[Object(c.jsx)("circle",{cx:62.5,cy:62.5,r:62,fill:"#91125c",stroke:"#91125c",strokeMiterlimit:10,"data-name":"Layer 1"}),Object(c.jsx)("text",{transform:"translate(45.27 94.56)",fontSize:84,fontFamily:"FuturaBT-MediumCondensed,Futura Condensed BT",fontWeight:500,children:"B"})]}))};var lt=Object(D.a)((function(t){return Object(M.a)({root:{display:"flex"},drawer:Object(u.a)({},t.breakpoints.up("sm"),{width:240,flexShrink:0}),appBar:Object(u.a)({},t.breakpoints.up("sm"),{zIndex:t.zIndex.drawer+1}),menuButton:Object(u.a)({marginRight:t.spacing(2)},t.breakpoints.up("sm"),{display:"none"}),toolbar:t.mixins.toolbar,drawerPaper:{width:240},content:{flexGrow:1,padding:t.spacing(3)}})}));function jt(t){var e=Object(w.a)(),n=e.register,a=e.handleSubmit;e.errors;return Object(c.jsx)("div",{className:"modal-box",children:Object(c.jsxs)("div",{className:"modal-text",children:[Object(c.jsx)("h2",{children:"Load a Game"}),Object(c.jsxs)("form",{onSubmit:a(t.onSubmit),className:"form",children:[Object(c.jsx)("input",{required:!0,type:"file",name:"rom",ref:n}),Object(c.jsx)("button",{children:"Submit"})]})]})})}function ut(t){return Object(c.jsxs)("div",{className:"gamepad",children:[Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("upButton",n)},defaultPosition:{x:t.locations.upButton.x,y:t.locations.upButton.y},children:Object(c.jsx)("div",{className:"up",children:Object(c.jsx)(nt,{className:"icon-button",onClick:function(){return t.onClick(a.DUp)}})})}),Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("leftButton",n)},defaultPosition:{x:t.locations.leftButton.x,y:t.locations.leftButton.y},children:Object(c.jsx)("div",{className:"left",children:Object(c.jsx)(at,{className:"icon-button",onClick:function(){return t.onClick(a.DLeft)}})})}),Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("rightButton",n)},defaultPosition:{x:t.locations.rightButton.x,y:t.locations.rightButton.y},children:Object(c.jsx)("div",{className:"right",children:Object(c.jsx)(ct,{className:"icon-button",onClick:function(){return t.onClick(a.DRight)}})})}),Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("downButton",n)},defaultPosition:{x:t.locations.downButton.x,y:t.locations.downButton.y},children:Object(c.jsx)("div",{className:"down",children:Object(c.jsx)(et,{className:"icon-button",onClick:function(){return t.onClick(a.DDown)}})})}),Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("select",n)},defaultPosition:{x:t.locations.select.x,y:t.locations.select.y},children:Object(c.jsx)("div",{className:"select",children:Object(c.jsx)(ot,{className:"start-select-button",onClick:function(){return t.onClick(a.Select)}})})}),Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("start",n)},defaultPosition:{x:t.locations.start.x,y:t.locations.start.y},children:Object(c.jsx)("div",{className:"start",children:Object(c.jsx)(rt,{className:"start-select-button",onClick:function(){return t.onClick(a.Start)}})})}),Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("a",n)},defaultPosition:{x:t.locations.a.x,y:t.locations.a.y},children:Object(c.jsx)("div",{className:"a-button",children:Object(c.jsx)(st,{className:"icon-button",onClick:function(){return t.onClick(a.A)}})})}),Object(c.jsx)(y.a,{disabled:t.disabled,grid:[10,10],onStop:function(e,n){return t.onStop("b",n)},defaultPosition:{x:t.locations.b.x,y:t.locations.b.y},children:Object(c.jsx)("div",{className:"b-button",children:Object(c.jsx)(it,{className:"icon-button",onClick:function(){return t.onClick(a.B)}})})})]})}function bt(t){var e=t.text,n=t.onClick;return Object(c.jsxs)(z.a,{onClick:n,button:!0,children:[Object(c.jsx)(G.a,{children:t.children}),Object(c.jsx)(T.a,{primary:e})]},e)}function dt(t){var e=lt(),n=r.a.useState(!1),a=Object(j.a)(n,2),o=a[0],s=a[1],i=function(){s(!o)},l=window,u=Object(c.jsxs)("div",{children:[Object(c.jsx)("div",{className:e.toolbar}),Object(c.jsx)(F.a,{}),Object(c.jsxs)(E.a,{children:[Object(c.jsx)(bt,{onClick:function(){return l.emu.load_save_state()},text:"Load",children:Object(c.jsx)(H.a,{})}),Object(c.jsx)(bt,{onClick:function(){return l.emu.make_save_state()},text:"Save",children:Object(c.jsx)(J.a,{})}),Object(c.jsx)(bt,{text:"Settings",children:Object(c.jsx)(K.a,{})}),Object(c.jsxs)(z.a,{button:!0,children:[Object(c.jsx)(G.a,{children:Object(c.jsx)(Q.a,{})}),Object(c.jsx)(T.a,{primary:"Configure GamePad"}),Object(c.jsx)(Z.a,{checked:!t.toggle,color:"default",onChange:t.onGamepadChange})]},"Configure GamePad"),Object(c.jsxs)(bt,{onClick:t.onKeyboardChange,text:"Configure Keyboard",children:[Object(c.jsx)(Y.a,{})," ",t.modal," "]}),Object(c.jsx)(bt,{text:"Delete User Data",onClick:t.onDelete,children:Object(c.jsx)(tt.a,{})})]})]});return Object(c.jsxs)("div",{className:e.root,children:[Object(c.jsx)(R.a,{}),Object(c.jsx)(k.a,{position:"fixed",className:e.appBar,children:Object(c.jsxs)(S.a,{children:[Object(c.jsx)(C.a,{color:"inherit","aria-label":"open drawer",edge:"start",onClick:i,className:e.menuButton,children:Object(c.jsx)(L.a,{})}),Object(c.jsx)(B.a,{variant:"h6",noWrap:!0,children:t.name})]})}),Object(c.jsxs)("nav",{className:e.drawer,"aria-label":"mailbox folders",children:[Object(c.jsx)(A.a,{smUp:!0,implementation:"css",children:Object(c.jsx)(I.a,{variant:"temporary",anchor:"left",open:o,onClose:i,classes:{paper:e.drawerPaper},ModalProps:{keepMounted:!0},children:u})}),Object(c.jsx)(A.a,{xsDown:!0,implementation:"css",children:Object(c.jsx)(I.a,{classes:{paper:e.drawerPaper},variant:"permanent",open:!0,children:u})})]}),Object(c.jsx)("main",{className:e.content})]})}var xt=function(){var t=window,e=Object(w.a)(),n=e.register,r=e.handleSubmit,s=Object(o.useState)(new p),i=Object(j.a)(s,2),u=(i[0],i[1],Object(o.useState)({name:"No File Selected"})),b=Object(j.a)(u,2),d=b[0],x=b[1],O=Object(o.useState)(!0),h=Object(j.a)(O,2),m=h[0],f=h[1],v=Object(o.useState)(!0),y=Object(j.a)(v,2),k=y[0],S=y[1],B=Object(o.useState)(JSON.parse(localStorage.getItem("controls"))||{up:"w",left:"a",down:"s",right:"d",a:"j",b:"k",start:" ",select:"b"}),C=Object(j.a)(B,2),N=C[0],L=C[1],D=Object(o.useState)(JSON.parse(localStorage.getItem("gamePadLocations"))||{upButton:{x:null,y:null},downButton:{x:null,y:null},leftButton:{x:null,y:null},rightButton:{x:null,y:null},start:{x:null,y:null},select:{x:null,y:null},a:{x:null,y:null},b:{x:null,y:null}}),M=Object(j.a)(D,2),I=M[0],A=M[1],E=function(t){switch(t){case N.up:return a.DUp;case N.left:return a.DLeft;case N.right:return a.DRight;case N.down:return a.DDown;case N.a:return a.A;case N.b:return a.B;case N.start:return a.Start;case N.select:return a.Select;default:return}},z=function(e){var n=E(e.key);void 0!==n&&t.button_down(n)},G=function(e){var n=E(e.key);void 0!==n&&t.button_up(n)},T=Object(o.useState)(!1),R=Object(j.a)(T,2),U=R[0],J=R[1],W=Object(c.jsx)(_.a,{open:U,onClose:function(){J(!1)},disableEnforceFocus:!0,children:Object(c.jsxs)("div",{className:"modal-box",children:[Object(c.jsx)("h2",{children:"Set Key Bindings"}),Object(c.jsxs)("form",{onSubmit:r((function(t){var e=function(t,e){switch(t){case"up":L(Object(l.a)(Object(l.a)({},N),{},{up:e}));break;case"down":L(Object(l.a)(Object(l.a)({},N),{},{down:e}));break;case"left":L(Object(l.a)(Object(l.a)({},N),{},{left:e}));break;case"right":L(Object(l.a)(Object(l.a)({},N),{},{right:e}));break;case"a":L(Object(l.a)(Object(l.a)({},N),{},{a:e}));break;case"b":L(Object(l.a)(Object(l.a)({},N),{},{b:e}));break;case"start":L(Object(l.a)(Object(l.a)({},N),{},{start:e}));break;case"select":L(Object(l.a)(Object(l.a)({},N),{},{select:e}));break;default:console.log("No such control exists")}};for(var n in t)0!==t[n].length&&e(n,t[n]);J(!1)})),children:[Object(c.jsx)("input",{name:"up",placeholder:"up",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("input",{name:"down",placeholder:"down",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("input",{name:"left",placeholder:"left",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("input",{name:"right",placeholder:"right",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("input",{name:"a",placeholder:"a",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("input",{name:"b",placeholder:"b",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("input",{name:"start",placeholder:"start",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("input",{name:"select",placeholder:"select",ref:n})," ",Object(c.jsx)("br",{}),Object(c.jsx)("button",{children:"Save"})]})]})});return Object(o.useEffect)((function(){return window.addEventListener("keydown",z),window.addEventListener("keyup",G),function(){window.removeEventListener("keydown",z),window.removeEventListener("keyup",G)}}),[]),Object(o.useEffect)((function(){return localStorage.setItem("gamePadLocations",JSON.stringify(I)),function(){localStorage.setItem("gamePadLocations",JSON.stringify(I))}}),[I]),Object(o.useEffect)((function(){localStorage.setItem("controls",JSON.stringify(N))}),[N]),Object(c.jsxs)("div",{className:"App",children:[Object(c.jsx)(_.a,{open:m,children:Object(c.jsx)(jt,{onSubmit:function(t){f(!1),x(t.rom[0])}})}),Object(c.jsx)(dt,{name:d.name,onGamepadChange:function(){S(!k)},onKeyboardChange:function(){J(!0)},onDelete:function(){localStorage.clear()},toggle:k,modal:W}),Object(c.jsxs)(P.a,{direction:"column",justify:"center",alignItems:"center",children:[Object(c.jsx)(P.a,{item:!0,children:Object(c.jsx)("div",{className:"screen",children:d.constructor===File?Object(c.jsx)(g,{id:"gb-emulator",rom:d}):Object(c.jsx)("p",{children:"Waiting for ROM"})})}),Object(c.jsx)(F.a,{}),Object(c.jsx)(P.a,{item:!0,children:Object(c.jsx)(ut,{onClick:function(e){var n;t.button_down(e),(n=85,new Promise((function(t){return setTimeout(t,n)}))).then((function(){t.button_up(e)}))},disabled:k,onStop:function(t,e){switch(t){case"upButton":A(Object(l.a)(Object(l.a)({},I),{},{upButton:{x:e.x,y:e.y}}));break;case"downButton":A(Object(l.a)(Object(l.a)({},I),{},{downButton:{x:e.x,y:e.y}}));break;case"leftButton":A(Object(l.a)(Object(l.a)({},I),{},{leftButton:{x:e.x,y:e.y}}));break;case"rightButton":A(Object(l.a)(Object(l.a)({},I),{},{rightButton:{x:e.x,y:e.y}}));break;case"start":A(Object(l.a)(Object(l.a)({},I),{},{start:{x:e.x,y:e.y}}));break;case"select":A(Object(l.a)(Object(l.a)({},I),{},{select:{x:e.x,y:e.y}}));break;case"a":A(Object(l.a)(Object(l.a)({},I),{},{a:{x:e.x,y:e.y}}));break;case"b":A(Object(l.a)(Object(l.a)({},I),{},{b:{x:e.x,y:e.y}}));break;default:console.log("No such button exists")}},locations:I})})]})]})},Ot=function(t){t&&t instanceof Function&&n.e(5).then(n.bind(null,154)).then((function(e){var n=e.getCLS,a=e.getFID,c=e.getFCP,o=e.getLCP,r=e.getTTFB;n(t),a(t),c(t),o(t),r(t)}))};i.a.render(Object(c.jsxs)(r.a.StrictMode,{children:[Object(c.jsx)("link",{rel:"stylesheet",href:"https://fonts.googleapis.com/css?family=Roboto:300,400,500,700&display=swap"}),Object(c.jsx)(xt,{})]}),document.getElementById("root")),Ot()}},[[96,2,3]]]);
//# sourceMappingURL=main.d06216cf.chunk.js.map