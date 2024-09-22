// ------------------- altv-esbuild banner -------------------
// banner imports
import ___altvEsbuild_altvInject_altShared___ from "alt-shared";
import ___altvEsbuild_altvInject_alt___ from "alt";
import ___altvEsbuild_altvInject_native___ from "natives";
const ___altvEsbuild_altvInject_pluginOptions___ = {"mode":"client","dev":{"enabled":false,"hotReload":false,"hotReloadServerPort":-1,"hotReloadServerHost":"","playersReconnect":false,"playersReconnectDelay":-1,"playersReconnectResetPos":false,"connectionCompleteEvent":false,"disconnectEvent":false,"restartCommand":false,"topLevelExceptionHandling":false,"moveExternalsOnTop":false,"enhancedRestartCommand":false,"serverStartedEvent":false,"clientServerInstanceValidation":false},"bugFixes":{"webViewFlickering":true,"playerPrototype":true},"altvEnums":false,"enhancedAltLog":true,"altDefaultImport":false};
await (async () => { // start banner wrapper
var ln=Object.defineProperty;var cn=(t,e,n)=>e in t?ln(t,e,{enumerable:!0,configurable:!0,writable:!0,value:n}):t[e]=n;var G=(t,e,n)=>(cn(t,typeof e!="symbol"?e+"":e,n),n);var I="altv-esbuild",Q="__altv-esbuild-resource-control";var ue=class{receiver;sender;constructor(e,n,r){this.isCommunicatorSenderAndReceiver(e)?(this.sender=e.sender??null,this.receiver=e.receiver??null):(this.sender=e,this.receiver=e),this.receiver?.on("data",o=>{o=o.toString();for(let i of o.split("|"))if(i)try{let{event:s,args:a}=JSON.parse(i),l=n[s];if(!l){r(`received unknown event: ${s}`);return}l(...a)}catch(s){r(`failed to handle chunk: '${i}' error: ${s?.stack}`)}}),this.receiver?.on("error",o=>{o?.code==="ECONNRESET"||o?.code==="ECONNREFUSED"||r(`socket error: ${o.stack}`,o)})}send(e,...n){if(!this.sender)throw new Error("EventManager cannot send since sender was not provided");let r={args:n,event:e};this.sender.write(JSON.stringify(r)+"|")}destroy(){this.receiver?.removeAllListeners("data"),this.receiver?.removeAllListeners("error")}isCommunicatorSenderAndReceiver(e){return!!(e.sender||e.receiver)}};var Ce=___altvEsbuild_altvInject_alt___,x=class{constructor(e){this.name=e;this.debug=()=>{}}debug;info(...e){Ce.log(`~bl~[${I}][${this.name}]~w~`,...e)}error(...e){Ce.logError(`[${I}][${this.name}]`,...e)}warn(...e){Ce.logWarning(`[${I}][${this.name}]`,...e)}};var g={},fn=/\u001b\[\d\d?m/g,{defineProperty:M,getOwnPropertyDescriptor:U,ownKeys:q}=Reflect,{apply:dn,bind:Pt,call:un}=Function.prototype,W=Pt.bind(un);g.uncurryThis=W;var be=Pt.bind(dn);g.applyBind=be;var We=["ArrayOf","ArrayPrototypePush","ArrayPrototypeUnshift","MathHypot","MathMax","MathMin","StringPrototypeConcat","TypedArrayOf"];function He(t){return typeof t=="symbol"?`Symbol${t.description[7].toUpperCase()}${t.description.slice(8)}`:`${t[0].toUpperCase()}${t.slice(1)}`}function Je(t,e,n,{enumerable:r,get:o,set:i}){M(t,`${e}Get${n}`,{__proto__:null,value:W(o),enumerable:r}),i!==void 0&&M(t,`${e}Set${n}`,{__proto__:null,value:W(i),enumerable:r})}function $t(t,e,n){for(let r of q(t)){let o=He(r),i=U(t,r);if("get"in i)Je(e,n,o,i);else{let s=`${n}${o}`;M(e,s,{__proto__:null,...i}),We.includes(s)&&M(e,`${s}Apply`,{__proto__:null,value:be(i.value,t)})}}}function pn(t,e,n){for(let r of q(t)){let o=He(r),i=U(t,r);if("get"in i)Je(e,n,o,i);else{let{value:s}=i;typeof s=="function"&&(i.value=s.bind(t));let a=`${n}${o}`;M(e,a,{__proto__:null,...i}),We.includes(a)&&M(e,`${a}Apply`,{__proto__:null,value:be(s,t)})}}}function ge(t,e,n){for(let r of q(t)){let o=He(r),i=U(t,r);if("get"in i)Je(e,n,o,i);else{let{value:s}=i;typeof s=="function"&&(i.value=W(s));let a=`${n}${o}`;M(e,a,{__proto__:null,...i}),We.includes(a)&&M(e,`${a}Apply`,{__proto__:null,value:be(s)})}}}["Proxy","globalThis"].forEach(t=>{g[t]=globalThis[t]});[decodeURI,decodeURIComponent,encodeURI,encodeURIComponent].forEach(t=>{g[t.name]=t});[escape,eval,unescape].forEach(t=>{g[t.name]=t});["JSON","Math","Proxy","Reflect"].forEach(t=>{$t(globalThis[t],g,t)});["AggregateError","Array","ArrayBuffer","BigInt","BigInt64Array","BigUint64Array","Boolean","DataView","Date","Error","EvalError","FinalizationRegistry","Float32Array","Float64Array","Function","Int16Array","Int32Array","Int8Array","Map","Number","Object","RangeError","ReferenceError","RegExp","Set","String","Symbol","SyntaxError","TypeError","URIError","Uint16Array","Uint32Array","Uint8Array","Uint8ClampedArray","WeakMap","WeakRef","WeakSet"].forEach(t=>{let e=globalThis[t];g[t]=e,$t(e,g,t),ge(e.prototype,g,`${t}Prototype`)});["Promise"].forEach(t=>{let e=globalThis[t];g[t]=e,pn(e,g,t),ge(e.prototype,g,`${t}Prototype`)});[{name:"TypedArray",original:Reflect.getPrototypeOf(Uint8Array)},{name:"ArrayIterator",original:{prototype:Reflect.getPrototypeOf(Array.prototype[Symbol.iterator]())}},{name:"StringIterator",original:{prototype:Reflect.getPrototypeOf(String.prototype[Symbol.iterator]())}}].forEach(({name:t,original:e})=>{g[t]=e,ge(e,g,t),ge(e.prototype,g,`${t}Prototype`)});var{ArrayPrototypeForEach:Ze=Array.prototype.forEach.call,FinalizationRegistry:Ne=Ne,FunctionPrototypeCall:qe=Function.prototype.call,Map:B=B,ObjectFreeze:oe=S.freeze.call,ObjectSetPrototypeOf:ie=S.setPrototypeOf,Promise:H=H,PromisePrototypeThen:kt=H.prototype.then,Set:K=K,SymbolIterator:se=Symbol.iterator,WeakMap:ye=ye,WeakRef:Le=Le,WeakSet:he=he}=g,Ye=(t,e)=>{class n{constructor(o){this._iterator=t(o)}next(){return e(this._iterator)}[se](){return this}}return ie(n.prototype,null),oe(n.prototype),oe(n),n};g.SafeArrayIterator=Ye(g.ArrayPrototypeSymbolIterator,g.ArrayIteratorPrototypeNext);g.SafeStringIterator=Ye(g.StringPrototypeSymbolIterator,g.StringIteratorPrototypeNext);var pt=(t,e)=>{Ze(q(t),n=>{U(e,n)||M(e,n,{__proto__:null,...U(t,n)})})},F=(t,e)=>{if(se in t.prototype){let n=new t,r;Ze(q(t.prototype),o=>{if(!U(e.prototype,o)){let i=U(t.prototype,o);if(typeof i.value=="function"&&i.value.length===0&&se in(qe(i.value,n)??{})){let s=W(i.value);r??=W(s(n).next);let a=Ye(s,r);i.value=function(){return new a(this)}}M(e.prototype,o,{__proto__:null,...i})}})}else pt(t.prototype,e.prototype);return pt(t,e),ie(e.prototype,null),oe(e.prototype),oe(e),e};g.makeSafe=F;g.SafeMap=F(B,class extends B{constructor(e){super(e)}});g.SafeWeakMap=F(ye,class extends ye{constructor(e){super(e)}});g.SafeSet=F(K,class extends K{constructor(e){super(e)}});g.SafeWeakSet=F(he,class extends he{constructor(e){super(e)}});g.SafeFinalizationRegistry=F(Ne,class extends Ne{constructor(e){super(e)}});g.SafeWeakRef=F(Le,class extends Le{constructor(e){super(e)}});var gn=F(H,class extends H{constructor(e){super(e)}});g.PromisePrototypeCatch=(t,e)=>kt(t,void 0,e);g.SafePromisePrototypeFinally=(t,e)=>new H((n,r)=>new gn((o,i)=>kt(t,o,i)).finally(e).then(n,r));g.AsyncIteratorPrototype=g.ReflectGetPrototypeOf(g.ReflectGetPrototypeOf(async function*(){}).prototype);ie(g,null);oe(g);var{getOwnNonIndexProperties:Be,getProxyDetails:yn=()=>{},kPending:Fo=0,kFulfilled:Do=1,kRejected:jo=2,previewEntries:Ee,getConstructorName:Xe,propertyFilter:{ALL_PROPERTIES:Rt,ONLY_ENUMERABLE:Ct}={ALL_PROPERTIES:0,ONLY_ENUMERABLE:2}}={},hn=(t,e)=>{if(e===Rt)return S.getOwnPropertyNames(t);if(Ct)return Array.isArray(t)?[]:S.keys(t);throw new Error("unknown filter")};Be=hn;var mn=t=>t?.constructor?.name??"<UNKNOWN CONSTRUCTOR NAME>";Xe=mn;var vn=t=>{let e=t instanceof B,n=[];if(!(t instanceof B||t instanceof K))return[n,e];try{n=t.entries()}catch(r){console.error("custom_previewEntries",r.stack)}return[n,e]};Ee=vn;var Te=Uint8Array.prototype.__proto__,{ArrayIsArray:It=Array.isArray,ArrayPrototypeFilter:Ot=Array.prototype.filter.call,ArrayPrototypePop:Sn=Array.prototype.pop.call,ArrayPrototypePush:L=Array.prototype.push.call,ArrayPrototypePushApply:gt=Array.prototype.push.apply,ArrayPrototypeSort:bn=Array.prototype.sort.call,ArrayPrototypeUnshift:yt=Array.prototype.unshift.call,BigIntPrototypeValueOf:En=BigInt.prototype.valueOf.call,BooleanPrototypeValueOf:_n=Boolean.prototype.valueOf.call,DatePrototypeGetTime:wn=Date.prototype.getTime.call,DatePrototypeToISOString:An=Date.prototype.toISOString.call,DatePrototypeToString:Pn=Date.prototype.toString.call,ErrorPrototypeToString:$n=Error.prototype.toString.call,FunctionPrototypeToString:kn=Function.prototype.toString.call,JSONStringify:zo=JSON.stringify,MapPrototypeGetSize:Rn=S.getOwnPropertyDescriptor(B.prototype,"size").get.call,MapPrototypeEntries:Cn=B.prototype.entries.call,MathFloor:In=Math.floor,MathMax:le=Math.max,MathMin:T=Math.min,MathRound:On=Math.round,MathSqrt:ht=Math.sqrt,MathTrunc:Mn=Math.trunc,Number:re=re,NumberIsFinite:xn=re.isFinite,NumberIsNaN:Mt=isNaN,NumberParseFloat:Uo=parseFloat,NumberParseInt:Ko=parseInt,NumberPrototypeValueOf:Nn=re.prototype.valueOf.call,Object:S=S,ObjectAssign:Qe=S.assign,ObjectCreate:xt=S.create,ObjectDefineProperty:et=S.defineProperty,ObjectGetOwnPropertyDescriptor:me=S.getOwnPropertyDescriptor,ObjectGetOwnPropertyNames:Fe=S.getOwnPropertyNames,ObjectGetOwnPropertySymbols:Ln=S.getOwnPropertySymbols,ObjectGetPrototypeOf:tt=S.getPrototypeOf,ObjectIs:Bn=S.is,ObjectKeys:_e=S.keys,ObjectPrototypeHasOwnProperty:ce=S.prototype.hasOwnProperty.call,ObjectPrototypePropertyIsEnumerable:Nt=S.prototype.propertyIsEnumerable.call,ObjectSeal:Tn=S.seal,RegExp:z=z,RegExpPrototypeExec:nt=z.prototype.exec.call,RegExpPrototypeSymbolReplace:ve=z.prototype[Symbol.replace].call,RegExpPrototypeToString:Fn=z.prototype.toString.call,SafeStringIterator:Dn,SafeMap:jn,SafeSet:Lt,SetPrototypeGetSize:zn=S.getOwnPropertyDescriptor(K.prototype,"size").get.call,SetPrototypeValues:Un=K.prototype.values.call,StringPrototypeCharCodeAt:De=String.prototype.charCodeAt.call,StringPrototypeCodePointAt:Kn=String.prototype.codePointAt.call,StringPrototypeIncludes:te=String.prototype.includes.call,StringPrototypeNormalize:Vn=String.prototype.normalize.call,StringPrototypePadEnd:Gn=String.prototype.padEnd.call,StringPrototypePadStart:Ie=String.prototype.padStart.call,StringPrototypeRepeat:mt=String.prototype.repeat.call,StringPrototypeSlice:Oe=String.prototype.slice.call,StringPrototypeSplit:Vo=String.prototype.split.call,StringPrototypeToLowerCase:Wn=String.prototype.toLowerCase.call,StringPrototypeTrim:Hn=String.prototype.trim.call,StringPrototypeValueOf:Jn=String.prototype.valueOf.call,SymbolPrototypeToString:Bt=Symbol.prototype.toString.call,SymbolPrototypeValueOf:Zn=Symbol.prototype.valueOf.call,SymbolToStringTag:vt=Symbol.toStringTag,TypedArrayPrototypeGetLength:qn=S.getOwnPropertyDescriptor(Te,"length").get.call,TypedArrayPrototypeGetSymbolToStringTag:Yn=S.getOwnPropertyDescriptor(Te,Symbol.toStringTag).get.call}=g,{customInspectSymbol:Tt=Symbol.for("nodejs.util.inspect.custom"),isError:Ft=t=>t instanceof Error,join:ne,removeColors:Dt}={};function Xn(t,e){let n="";if(t.length!==0){let r=t.length-1;for(let o=0;o<r;o++)n+=t[o],n+=e;n+=t[r]}return n}ne=Xn;function Qn(t){return String.prototype.replace.call(t,fn,"")}Dt=Qn;var{isStackOverflowError:jt}={};function N(t){if(N.maxStack_ErrorMessage===void 0)try{let n=function(){n()};var e=n;n()}catch(n){N.maxStack_ErrorMessage=n.message,N.maxStack_ErrorName=n.name}return t&&t.name===N.maxStack_ErrorName&&t.message===N.maxStack_ErrorMessage}N.maxStack_ErrorMessage=void 0;N.maxStack_ErrorName=void 0;jt=N;var{isAsyncFunction:er=t=>t?.constructor?.name==="AsyncFunction",isGeneratorFunction:tr=t=>t?.constructor?.name==="GeneratorFunction",isAnyArrayBuffer:nr=t=>t instanceof ArrayBuffer||typeof SharedArrayBuffer<"u"&&t instanceof SharedArrayBuffer,isArrayBuffer:zt,isArgumentsObject:Ut,isBoxedPrimitive:Kt,isDataView:Vt,isExternal:rr=()=>!1,isMap:or=t=>t instanceof B,isMapIterator:ir=t=>t?.toString()==="[object Map Iterator]",isModuleNamespaceObject:rt,isNativeError:Gt=t=>!1,isPromise:sr=t=>t instanceof H,isSet:ar=t=>t instanceof K,isSetIterator:lr=t=>t?.toString()==="[object Set Iterator]",isWeakMap:cr=t=>t instanceof ye,isWeakSet:fr=t=>t instanceof he,isRegExp:dr=t=>t instanceof z,isDate:ur=t=>t instanceof Date,isTypedArray:pr=t=>t instanceof Te.constructor,isStringObject:gr=t=>typeof t=="object"&&t!=null&&t.constructor===String,isNumberObject:yr=t=>typeof t=="object"&&t!=null&&t.constructor===re,isBooleanObject:hr=t=>typeof t=="object"&&t!=null&&t.constructor===Boolean,isBigIntObject:mr=t=>typeof t=="object"&&t!=null&&t.constructor===BigInt}={},vr=t=>t instanceof ArrayBuffer||typeof t=="object"&&t.constructor&&t.constructor.name==="ArrayBuffer"&&t.byteLength>=0;zt=vr;var Sr=t=>t+""=="[object Arguments]"&&t[se]!=null;Ut=Sr;var br=t=>typeof t=="object"&&t!=null&&(t.constructor===re||t.constructor===String||t.constructor===Boolean||t.constructor===BigInt||t.constructor===Symbol);Kt=br;var Er=t=>t instanceof DataView;Vt=Er;var _r=t=>{try{let e=t&&S.getOwnPropertyDescriptor(t,Symbol.toStringTag);return e.value==="Module"&&!(e.writable||e.enumerable||e.configurable)}catch{return!1}};rt=_r;var je=class extends Error{};function Wt(t,e){if(!t)throw new je(e)}var wr={exists:()=>!1};function Ar(t){let e="__internal_shit__"+t.name;return et(t,"name",{__proto__:null,value:e}),t}var Pr=Ar((t,e,n)=>{let r=n==null,o=r?!1:n.allowArray,i=r?!1:n.allowFunction;if(!(r?!1:n.nullable)&&t===null||!o&&It(t)||typeof t!="object"&&(!i||typeof t!="function"))throw new Error(`[validateObject] invalid ${e} type of arg, expected: Object`)});function $r(t,e){if(typeof t!="string")throw new Error(`value ${e} must be string`)}var Me,Ht=new Lt(Ot(Fe(globalThis),t=>nt(/^[A-Z][a-zA-Z0-9]+$/,t)!==null)),Jt=t=>typeof t>"u"&&t!==void 0,_=Tn({showHidden:!1,depth:2,colors:!1,customInspect:!0,showProxy:!1,maxArrayLength:100,maxStringLength:1e4,breakLength:80,compact:3,sorted:!1,getters:!1,numericSeparator:!1}),J=0,ot=1,ae=2,kr=/[\x00-\x1f\x27\x5c\x7f-\x9f]|[\ud800-\udbff](?![\udc00-\udfff])|(?<![\ud800-\udbff])[\udc00-\udfff]/,ze=/[\x00-\x1f\x27\x5c\x7f-\x9f]|[\ud800-\udbff](?![\udc00-\udfff])|(?<![\ud800-\udbff])[\udc00-\udfff]/g,Rr=/[\x00-\x1f\x5c\x7f-\x9f]|[\ud800-\udbff](?![\udc00-\udfff])|(?<![\ud800-\udbff])[\udc00-\udfff]/,Cr=/[\x00-\x1f\x5c\x7f-\x9f]|[\ud800-\udbff](?![\udc00-\udfff])|(?<![\ud800-\udbff])[\udc00-\udfff]/g,Ir=/^[a-zA-Z_][a-zA-Z_0-9]*$/,Or=/^(0|[1-9][0-9]*)$/,Mr=/^ {4}at (?:[^/\\(]+ \(|)node:(.+):\d+:\d+\)?$/,xr=/[/\\]node_modules[/\\](.+?)(?=[/\\])/g,Nr=/^(\s+[^(]*?)\s*{/,Lr=/(\/\/.*?\n)|(\/\*(.|\n)*?\*\/)/g,Br=16,we=0,Tr=1,Fr=2,Se=["\\x00","\\x01","\\x02","\\x03","\\x04","\\x05","\\x06","\\x07","\\b","\\t","\\n","\\x0B","\\f","\\r","\\x0E","\\x0F","\\x10","\\x11","\\x12","\\x13","\\x14","\\x15","\\x16","\\x17","\\x18","\\x19","\\x1A","\\x1B","\\x1C","\\x1D","\\x1E","\\x1F","","","","","","","","\\'","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","\\\\","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","\\x7F","\\x80","\\x81","\\x82","\\x83","\\x84","\\x85","\\x86","\\x87","\\x88","\\x89","\\x8A","\\x8B","\\x8C","\\x8D","\\x8E","\\x8F","\\x90","\\x91","\\x92","\\x93","\\x94","\\x95","\\x96","\\x97","\\x98","\\x99","\\x9A","\\x9B","\\x9C","\\x9D","\\x9E","\\x9F"],Dr="[\\u001B\\u009B][[\\]()#;?]*(?:(?:(?:(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]+)*|[a-zA-Z\\d]+(?:;[-a-zA-Z\\d\\/#&.:=?%@~_]*)*)?\\u0007)|(?:(?:\\d{1,4}(?:;\\d{0,4})*)?[\\dA-PR-TZcf-ntqry=><~]))",jr=new z(Dr,"g"),it;function zr(t,e){let n={stylize:t.stylize,showHidden:t.showHidden,depth:t.depth,colors:t.colors,customInspect:t.customInspect,showProxy:t.showProxy,maxArrayLength:t.maxArrayLength,maxStringLength:t.maxStringLength,breakLength:t.breakLength,compact:t.compact,sorted:t.sorted,getters:t.getters,numericSeparator:t.numericSeparator,...t.userOptions};if(e){ie(n,null);for(let r of _e(n))(typeof n[r]=="object"||typeof n[r]=="function")&&n[r]!==null&&delete n[r];n.stylize=ie((r,o)=>{let i;try{i=`${t.stylize(r,o)}`}catch{}return typeof i!="string"?r:i},null)}return n}function O(t,e){let n={budget:{},indentationLvl:0,seen:[],currentDepth:0,stylize:Ve,showHidden:_.showHidden,depth:_.depth,colors:_.colors,customInspect:_.customInspect,showProxy:_.showProxy,maxArrayLength:_.maxArrayLength,maxStringLength:_.maxStringLength,breakLength:_.breakLength,compact:_.compact,sorted:_.sorted,getters:_.getters,numericSeparator:_.numericSeparator};if(arguments.length>1){if(arguments.length>2&&(arguments[2]!==void 0&&(n.depth=arguments[2]),arguments.length>3&&arguments[3]!==void 0&&(n.colors=arguments[3])),typeof e=="boolean")n.showHidden=e;else if(e){let r=_e(e);for(let o=0;o<r.length;++o){let i=r[o];ce(_,i)||i==="stylize"?n[i]=e[i]:n.userOptions===void 0&&(n.userOptions=e)}}}return n.colors&&(n.stylize=Ur),n.maxArrayLength===null&&(n.maxArrayLength=1/0),n.maxStringLength===null&&(n.maxStringLength=1/0),A(n,t,0)}O.custom=Tt;et(O,"defaultOptions",{__proto__:null,get(){return _},set(t){return Pr(t,"options"),Qe(_,t)}});O.colors=Qe(xt(null),{reset:["w",0],bold:["wl"],dim:["w"],italic:["w"],underline:["w"],blink:["w"],inverse:["w"],hidden:["w"],strikethrough:["w"],doubleunderline:["w"],black:["k"],red:["rl"],green:["gl"],yellow:["yl"],blue:["bl"],magenta:["ml"],cyan:["cl"],white:["wl"],bgBlack:["w"],bgRed:["w"],bgGreen:["w"],bgYellow:["w"],bgBlue:["w"],bgMagenta:["w"],bgCyan:["w"],bgWhite:["w"],framed:["w"],overlined:["w"],gray:["kl"],redBright:["rl"],greenBright:["gl"],yellowBright:["yl"],blueBright:["bl"],magentaBright:["ml"],cyanBright:["cl"],whiteBright:["wl"],bgGray:["kl"],bgRedBright:["rl"],bgGreenBright:["gl"],bgYellowBright:["yl"],bgBlueBright:["bl"],bgMagentaBright:["ml"],bgCyanBright:["cl"],bgWhiteBright:["wl"]});function C(t,e){et(O.colors,e,{__proto__:null,get(){return this[t]},set(n){this[t]=n},configurable:!0,enumerable:!1})}C("gray","grey");C("gray","blackBright");C("bgGray","bgGrey");C("bgGray","bgBlackBright");C("dim","faint");C("strikethrough","crossedout");C("strikethrough","strikeThrough");C("strikethrough","crossedOut");C("hidden","conceal");C("inverse","swapColors");C("inverse","swapcolors");C("doubleunderline","doubleUnderline");O.styles=Qe(xt(null),{special:"cyan",number:"yellow",bigint:"yellow",boolean:"yellow",undefined:"grey",null:"bold",string:"green",symbol:"green",date:"magenta",regexp:"red",module:"underline"});function xe(t,e){return e===-1?`"${t}"`:e===-2?`\`${t}\``:`'${t}'`}function Ue(t){let e=De(t);return Se.length>e?Se[e]:`\\u${e.toString(16)}`}function Ke(t){let e=kr,n=ze,r=39;if(te(t,"'")&&(te(t,'"')?!te(t,"`")&&!te(t,"${")&&(r=-2):r=-1,r!==39&&(e=Rr,n=Cr)),t.length<5e3&&nt(e,t)===null)return xe(t,r);if(t.length>100)return t=ve(n,t,Ue),xe(t,r);let o="",i=0;for(let s=0;s<t.length;s++){let a=De(t,s);if(a===r||a===92||a<32||a>126&&a<160)i===s?o+=Se[a]:o+=`${Oe(t,i,s)}${Se[a]}`,i=s+1;else if(a>=55296&&a<=57343){if(a<=56319&&s+1<t.length){let l=De(t,s+1);if(l>=56320&&l<=57343){s++;continue}}o+=`${Oe(t,i,s)}${`\\u${a.toString(16)}`}`,i=s+1}}return i!==t.length&&(o+=Oe(t,i)),xe(o,r)}function Ur(t,e){let n=O.styles[e];if(n!==void 0){let r=O.colors[n];if(r!==void 0)return`~${r[0]}~${t}~w~`}return t}function Ve(t){return t}function Kr(){return[]}function Vr(t,e){try{return t instanceof e}catch{return!1}}function Zt(t,e,n,r){let o,i=t;for(;t||Jt(t);){let l=me(t,"constructor");if(l!==void 0&&typeof l.value=="function"&&l.value.name!==""&&Vr(i,l.value))return r!==void 0&&(o!==t||!Ht.has(l.value.name))&&Gr(e,i,o||i,n,r),String(l.value.name);t=tt(t),o===void 0&&(o=t)}if(o===null)return null;let s=Xe(i);if(n>e.depth&&e.depth!==null)return`${s} <Complex prototype>`;let a=Zt(o,e,n+1,r);return a===null?`${s} <${O(o,{...e,customInspect:!1,depth:-1})}>`:`${s} <${a}>`}function Gr(t,e,n,r,o){let i=0,s,a;do{if(i!==0||e===n){if(n=tt(n),n===null)return;let l=me(n,"constructor");if(l!==void 0&&typeof l.value=="function"&&Ht.has(l.value.name))return}i===0?a=new Lt:Ze(s,l=>a.add(l)),s=q(n),L(t.seen,e);for(let l of s){if(l==="constructor"||ce(e,l)||i!==0&&a.has(l))continue;let f=me(n,l);if(typeof f.value=="function")continue;let c=Z(t,n,r,l,J,f,e);t.colors?L(o,`\x1B[2m${c}\x1B[22m`):L(o,c)}Sn(t.seen)}while(++i!==3)}function w(t,e,n,r=""){return t===null?e!==""&&n!==e?`[${n}${r}: null prototype] [${e}] `:`[${n}${r}: null prototype] `:e!==""&&t!==e?`${t}${r} [${e}] `:`${t}${r} `}function ee(t,e){let n,r=Ln(t);if(e)n=Fe(t),r.length!==0&&gt(n,r);else{try{n=_e(t)}catch(o){Wt(Gt(o)&&o.name==="ReferenceError"&&rt(t)),n=Fe(t)}r.length!==0&&gt(n,Ot(r,i=>Nt(t,i)))}return n}function pe(t,e,n){let r="";return e===null&&(r=Xe(t),r===n&&(r="Object")),w(e,n,r)}function Wr(t,e,n){if(n>t.depth&&t.depth!==null)return t.stylize("Proxy [Array]","special");n+=1,t.indentationLvl+=2;let r=[A(t,e[0],n),A(t,e[1],n)];return t.indentationLvl-=2,lt(t,r,"",["Proxy [","]"],ae,n)}function A(t,e,n,r){if(typeof e!="object"&&typeof e!="function"&&!Jt(e))return at(t.stylize,e,t);if(e===null)return t.stylize("null","null");let o=e,i=yn(e,!!t.showProxy);if(i!==void 0){if(i===null||i[0]===null)return t.stylize("<Revoked Proxy>","special");if(t.showProxy)return Wr(t,i,n);e=i}if(t.customInspect){let s=e[Tt];if(typeof s=="function"&&s!==O&&!(e.constructor&&e.constructor.prototype===e)){let a=t.depth===null?null:t.depth-n,l=i!==void 0||!(o instanceof S),f=qe(s,o,a,zr(t,l),O);if(f!==o)return typeof f!="string"?A(t,f,n):f.replace(/\n/g,`
${" ".repeat(t.indentationLvl)}`)}}if(t.seen.includes(e)){let s=1;return t.circular===void 0?(t.circular=new jn,t.circular.set(e,s)):(s=t.circular.get(e),s===void 0&&(s=t.circular.size+1,t.circular.set(e,s))),t.stylize(`[Circular *${s}]`,"special")}return Hr(t,e,n,r)}function Hr(t,e,n,r){let o,i;t.showHidden&&(n<=t.depth||t.depth===null)&&(i=[]);let s=Zt(e,t,n,i);i!==void 0&&i.length===0&&(i=void 0);let a=e[vt];(typeof a!="string"||a!==""&&(t.showHidden?ce:Nt)(e,vt))&&(a="");let l="",f=Kr,c,p=!0,y=0,h=t.showHidden?Rt:Ct,E=J;if(e[se]||s===null)if(p=!1,It(e)){let u=s!=="Array"||a!==""?w(s,a,"Array",`(${e.length})`):"";if(o=Be(e,h),c=[`${u}[`,"]"],e.length===0&&o.length===0&&i===void 0)return`${c[0]}]`;E=ae,f=lo}else if(ar(e)){let u=zn(e),v=w(s,a,"Set",`(${u})`);if(o=ee(e,t.showHidden),f=s!==null?bt.bind(null,e):bt.bind(null,Un(e)),u===0&&o.length===0&&i===void 0)return`${v}{}`;c=[`${v}{`,"}"]}else if(or(e)){let u=Rn(e),v=w(s,a,"Map",`(${u})`);if(o=ee(e,t.showHidden),f=s!==null?Et.bind(null,e):Et.bind(null,Cn(e)),u===0&&o.length===0&&i===void 0)return`${v}{}`;c=[`${v}{`,"}"]}else if(pr(e)){o=Be(e,h);let u=e,v="";s===null&&(v=Yn(e),u=new g[v](e));let ut=qn(e);if(c=[`${w(s,a,v,`(${ut})`)}[`,"]"],e.length===0&&o.length===0&&!t.showHidden)return`${c[0]}]`;f=co.bind(null,u,ut),E=ae}else ir(e)?(o=ee(e,t.showHidden),c=St("Map",a),f=wt.bind(null,c)):lr(e)?(o=ee(e,t.showHidden),c=St("Set",a),f=wt.bind(null,c)):p=!0;if(p)if(o=ee(e,t.showHidden),c=["{","}"],s==="Object"){if(Ut(e)?c[0]="[Arguments] {":a!==""&&(c[0]=`${w(s,a,"Object")}{`),o.length===0&&i===void 0)return`${c[0]}}`}else if(typeof e=="function"){if(l=qr(e,s,a),o.length===0&&i===void 0)return t.stylize(l,"special")}else if(dr(e)){l=Fn(s!==null?e:new z(e));let u=w(s,a,"RegExp");if(u!=="RegExp "&&(l=`${u}${l}`),o.length===0&&i===void 0||n>t.depth&&t.depth!==null)return t.stylize(l,"regexp")}else if(ur(e)){l=Mt(wn(e))?Pn(e):An(e);let u=w(s,a,"Date");if(u!=="Date "&&(l=`${u}${l}`),o.length===0&&i===void 0)return t.stylize(l,"date")}else if(Ft(e)){if(l=to(e,s,a,t,o),o.length===0&&i===void 0)return l}else if(nr(e)){let u=zt(e)?"ArrayBuffer":"SharedArrayBuffer",v=w(s,a,u);if(r===void 0)f=ao;else if(o.length===0&&i===void 0)return v+`{ byteLength: ${st(t.stylize,e.byteLength,!1)} }`;c[0]=`${v}{`,yt(o,"byteLength")}else if(Vt(e))c[0]=`${w(s,a,"DataView")}{`,yt(o,"byteLength","byteOffset","buffer");else if(sr(e))c[0]=`${w(s,a,"Promise")}{`,f=po;else if(fr(e))c[0]=`${w(s,a,"WeakSet")}{`,f=t.showHidden?fo:_t;else if(cr(e))c[0]=`${w(s,a,"WeakMap")}{`,f=t.showHidden?uo:_t;else if(rt(e))c[0]=`${w(s,a,"Module")}{`,f=io.bind(null,o);else if(Kt(e)){if(l=Jr(e,t,o,s,a),o.length===0&&i===void 0)return l}else{if(o.length===0&&i===void 0){if(rr(e)){let u="UNSUPPORTED VALUE";return t.stylize(`[External: ${u}]`,"special")}return`${pe(e,s,a)}{}`}c[0]=`${pe(e,s,a)}{`}if(n>t.depth&&t.depth!==null){let u=pe(e,s,a).slice(0,-1);return s!==null&&(u=`[${u}]`),t.stylize(u,"special")}n+=1,t.seen.push(e),t.currentDepth=n;let k,X=t.indentationLvl;try{for(k=f(t,e,n),y=0;y<o.length;y++)k.push(Z(t,e,n,o[y],E));i!==void 0&&k.push(...i)}catch(u){let v=pe(e,s,a).slice(0,-1);return ro(t,u,v,X)}if(t.circular!==void 0){let u=t.circular.get(e);if(u!==void 0){let v=t.stylize(`<ref *${u}>`,"special");t.compact!==!0?l=l===""?v:`${v} ${l}`:c[0]=`${v} ${c[0]}`}}if(t.seen.pop(),t.sorted){let u=t.sorted===!0?void 0:t.sorted;if(E===J)k=k.sort(u);else if(o.length>1){let v=k.slice(k.length-o.length).sort(u);k.splice(k.length-o.length,o.length,...v)}}let b=lt(t,k,l,c,E,n,e),R=(t.budget[t.indentationLvl]||0)+b.length;return t.budget[t.indentationLvl]=R,R>2**27&&(t.depth=-1),b}function St(t,e){return e!==`${t} Iterator`&&(e!==""&&(e+="] ["),e+=`${t} Iterator`),[`[${e}] {`,"}"]}function Jr(t,e,n,r,o){let i,s;yr(t)?(i=Nn,s="Number"):gr(t)?(i=Jn,s="String",n.splice(0,t.length)):hr(t)?(i=_n,s="Boolean"):mr(t)?(i=En,s="BigInt"):(i=Zn,s="Symbol");let a=`[${s}`;return s!==r&&(r===null?a+=" (null prototype)":a+=` (${r})`),a+=`: ${at(Ve,i(t),e)}]`,o!==""&&o!==r&&(a+=` [${o}]`),n.length!==0||e.stylize===Ve?a:e.stylize(a,Wn(s))}function Zr(t,e,n){let i=`class ${ce(t,"name")&&t.name||"(anonymous)"}`;if(e!=="Function"&&e!==null&&(i+=` [${e}]`),n!==""&&e!==n&&(i+=` [${n}]`),e!==null){let s=tt(t).name;s&&(i+=` extends ${s}`)}else i+=" extends [null prototype]";return`[${i}]`}function qr(t,e,n){let r=kn(t);if(r.startsWith("class")&&r.endsWith("}")){let s=r.slice(5,-1),a=s.indexOf("{");if(a!==-1&&(!s.slice(0,a).includes("(")||Nr.test(s.replace(Lr))))return Zr(t,e,n)}let o="Function";tr(t)&&(o=`Generator${o}`),er(t)&&(o=`Async${o}`);let i=`[${o}`;return e===null&&(i+=" (null prototype)"),t.name===""?i+=" (anonymous)":i+=`: ${t.name}`,i+="]",e!==o&&e!==null&&(i+=` ${e}`),n!==""&&e!==n&&(i+=` [${n}]`),i}function Yr(t,e){for(let n=0;n<t.length-3;n++){let r=e.indexOf(t[n]);if(r!==-1){let o=e.length-r;if(o>3){let i=1,s=T(t.length-n,o);for(;s>i&&t[n+i]===e[r+i];)i++;if(i>3)return{len:i,offset:n}}}}return{len:0,offset:0}}function qt(t){return t.stack?String(t.stack):$n(t)}function Xr(t,e,n){let r=n.split(`
`);if(e.cause&&Ft(e.cause)){let o=qt(e.cause),i=o.indexOf(`
    at`);if(i!==-1){let s=o.slice(i+1).split(`
`),{len:a,offset:l}=Yr(r,s);if(a>0){let f=a-2,c=`    ... ${f} lines matching cause stack trace ...`;r.splice(l+1,f,t.stylize(c,"undefined"))}}}return r}function Qr(t,e,n,r){let o=n.length;if(e===null||n.endsWith("Error")&&t.startsWith(n)&&(t.length===o||t[o]===":"||t[o]===`
`)){let i="Error";if(e===null){let a=t.match(/^([A-Z][a-z_ A-Z0-9[\]()-]+)(?::|\n {4}at)/)||t.match(/^([a-z_A-Z0-9-]*Error)$/);i=a&&a[1]||"",o=i.length,i=i||"Error"}let s=w(e,r,i).slice(0,-1);n!==s&&(s.includes(n)?o===0?t=`${s}: ${t}`:t=`${s}${t.slice(o)}`:t=`${s} [${n}]${t.slice(o)}`)}return t}function eo(t,e,n,r){if(!t.showHidden&&e.length!==0)for(let o of["name","message","stack"]){let i=e.indexOf(o);i!==-1&&r.includes(n[o])&&e.splice(i,1)}}function to(t,e,n,r,o){let i=t.name!=null?String(t.name):"Error",s=qt(t);eo(r,o,t,s),"cause"in t&&(o.length===0||!o.includes("cause"))&&o.push("cause"),s=Qr(s,e,i,n);let a=t.message&&s.indexOf(t.message)||-1;a!==-1&&(a+=t.message.length);let l=s.indexOf(`
    at`,a);if(l===-1)s=`[${s}]`;else{let f=s.slice(0,l),c=Xr(r,t,s.slice(l+1));if(r.colors)for(let p of c){let y=p.match(Mr);if(y!==null&&wr.exists(y[1]))f+=`
${r.stylize(p,"undefined")}`;else{let h;f+=`
`;let E=0;for(;(h=xr.exec(p))!==null;)f+=p.slice(E,h.index+14),f+=r.stylize(h[1],"module"),E=h.index+h[0].length;f+=E===0?p:p.slice(E)}}else f+=`
${c.join(`
`)}`;s=f}if(r.indentationLvl!==0){let f=" ".repeat(r.indentationLvl);s=s.replace(/\n/g,`
${f}`)}return s}function no(t,e,n){let r=0,o=0,i=0,s=e.length;t.maxArrayLength<e.length&&s--;let a=2,l=new Array(s);for(;i<s;i++){let c=it(e[i],t.colors);l[i]=c,r+=c+a,o<c&&(o=c)}let f=o+a;if(f*3+t.indentationLvl<t.breakLength&&(r/f>5||o<=6)){let p=ht(f-r/e.length),y=le(f-3-p,1),h=T(On(ht(2.5*y*s)/y),In((t.breakLength-t.indentationLvl)/f),t.compact*4,15);if(h<=1)return e;let E=[],k=[];for(let b=0;b<h;b++){let j=0;for(let R=b;R<e.length;R+=h)l[R]>j&&(j=l[R]);j+=a,k[b]=j}let X=Ie;if(n!==void 0){for(let b=0;b<e.length;b++)if(typeof n[b]!="number"&&typeof n[b]!="bigint"){X=Gn;break}}for(let b=0;b<s;b+=h){let j=T(b+h,s),R="",u=b;for(;u<j-1;u++){let v=k[u-b]+e[u].length-l[u];R+=X(`${e[u]}, `,v," ")}if(X===Ie){let v=k[u-b]+e[u].length-l[u]-a;R+=Ie(e[u],v," ")}else R+=e[u];L(E,R)}t.maxArrayLength<e.length&&L(E,e[s]),e=E}return e}function ro(t,e,n,r){if(jt(e))return t.seen.pop(),t.indentationLvl=r,t.stylize(`[${n}: Inspection interrupted prematurely. Maximum call stack size exceeded.]`,"special");throw new Error(e.stack)}function Ge(t){let e="",n=t.length,r=t.startsWith("-")?1:0;for(;n>=r+4;n-=3)e=`_${t.slice(n-3,n)}${e}`;return n===t.length?t:`${t.slice(0,n)}${e}`}function oo(t){let e="",n=0;for(;n<t.length-3;n+=3)e+=`${t.slice(n,n+3)}_`;return n===0?t:`${e}${t.slice(n)}`}function st(t,e,n){if(!n)return Bn(e,-0)?t("-0","number"):t(`${e}`,"number");let r=Mn(e),o=String(r);return r===e?!xn(e)||o.includes("e")?t(o,"number"):t(`${Ge(o)}`,"number"):Mt(e)?t(o,"number"):t(`${Ge(o)}.${oo(String(e).slice(o.length+1))}`,"number")}function Yt(t,e,n){let r=String(e);return t(n?`${Ge(r)}n`:`${r}n`,"bigint")}function at(t,e,n){if(typeof e=="string"){let r="";if(e.length>n.maxStringLength){let o=e.length-n.maxStringLength;e=e.slice(0,n.maxStringLength),r=`... ${o} more character${o>1?"s":""}`}return n.compact!==!0&&e.length>Br&&e.length>n.breakLength-n.indentationLvl-4?e.split(/(?<=\n)/).map(o=>t(Ke(o),"string")).join(` +
${" ".repeat(n.indentationLvl+2)}`)+r:t(Ke(e),"string")+r}return typeof e=="number"?st(t,e,n.numericSeparator):typeof e=="bigint"?Yt(t,e,n.numericSeparator):typeof e=="boolean"?t(`${e}`,"boolean"):typeof e>"u"?t("undefined","undefined"):t(Bt(e),"symbol")}function io(t,e,n,r){let o=new Array(t.length);for(let i=0;i<t.length;i++)try{o[i]=Z(e,n,r,t[i],J)}catch(s){Wt(Gt(s)&&s.name==="ReferenceError");let a={[t[i]]:""};o[i]=Z(e,a,r,t[i],J);let l=o[i].lastIndexOf(" ");o[i]=o[i].slice(0,l+1)+e.stylize("<uninitialized>","special")}return t.length=0,o}function so(t,e,n,r,o,i){let s=_e(e),a=i;for(;i<s.length&&o.length<r;i++){let f=s[i],c=+f;if(c>2**32-2)break;if(`${a}`!==f){if(!Or.test(f))break;let p=c-a,y=p>1?"s":"",h=`<${p} empty item${y}>`;if(o.push(t.stylize(h,"undefined")),a=c,o.length===r)break}o.push(Z(t,e,n,f,ot)),a++}let l=e.length-a;if(o.length!==r){if(l>0){let f=l>1?"s":"",c=`<${l} empty item${f}>`;o.push(t.stylize(c,"undefined"))}}else l>0&&o.push(`... ${l} more item${l>1?"s":""}`);return o}function ao(t,e){let n;try{n=new Uint8Array(e)}catch{return[t.stylize("(detached)","special")]}Me===void 0&&(Me=function(s){return[...new Uint8Array(s)].map(a=>a.toString(16).padStart(2,"0")).join("")});let r=Hn(ve(/(.{2})/g,Me(n,0,T(t.maxArrayLength,n.length)),"$1 ")),o=n.length-t.maxArrayLength;return o>0&&(r+=` ... ${o} more byte${o>1?"s":""}`),[`${t.stylize("[Uint8Contents]","special")}: <${r}>`]}function lo(t,e,n){let r=e.length,o=T(le(0,t.maxArrayLength),r),i=r-o,s=[];for(let a=0;a<o;a++){if(!ce(e,a))return so(t,e,n,o,s,a);s.push(Z(t,e,n,a,ot))}return i>0&&s.push(`... ${i} more item${i>1?"s":""}`),s}function co(t,e,n,r,o){let i=T(le(0,n.maxArrayLength),e),s=t.length-i,a=new Array(i),l=t.length>0&&typeof t[0]=="number"?st:Yt;for(let f=0;f<i;++f)a[f]=l(n.stylize,t[f],n.numericSeparator);if(s>0&&(a[i]=`... ${s} more item${s>1?"s":""}`),n.showHidden){n.indentationLvl+=2;for(let f of["BYTES_PER_ELEMENT","length","byteLength","byteOffset","buffer"]){let c=A(n,t[f],o,!0);L(a,`[${f}]: ${c}`)}n.indentationLvl-=2}return a}function bt(t,e,n,r){let o=[];e.indentationLvl+=2;for(let i of t)L(o,A(e,i,r));return e.indentationLvl-=2,o}function Et(t,e,n,r){let o=[];e.indentationLvl+=2;for(let{0:i,1:s}of t)o.push(`${A(e,i,r)} => ${A(e,s,r)}`);return e.indentationLvl-=2,o}function Xt(t,e,n,r){let o=le(t.maxArrayLength,0),i=T(o,n.length),s=new Array(i);t.indentationLvl+=2;for(let l=0;l<i;l++)s[l]=A(t,n[l],e);t.indentationLvl-=2,r===we&&!t.sorted&&bn(s);let a=n.length-i;return a>0&&L(s,`... ${a} more item${a>1?"s":""}`),s}function Qt(t,e,n,r){let o=le(t.maxArrayLength,0),i=n.length/2,s=i-o,a=T(o,i),l=new Array(a),f=0;if(t.indentationLvl+=2,r===we){for(;f<a;f++){let c=f*2;l[f]=`${A(t,n[c],e)} => ${A(t,n[c+1],e)}`}t.sorted||(l=l.sort())}else for(;f<a;f++){let c=f*2,p=[A(t,n[c],e),A(t,n[c+1],e)];l[f]=lt(t,p,"",["[","]"],ae,e)}return t.indentationLvl-=2,s>0&&l.push(`... ${s} more item${s>1?"s":""}`),l}function _t(t){return[t.stylize("<items unknown>","special")]}function fo(t,e,n){let r=Ee(e);return Xt(t,n,r,we)}function uo(t,e,n){let r=Ee(e);return Qt(t,n,r,we)}function wt(t,e,n,r){let{0:o,1:i}=Ee(n,!0);return i?(t[0]=t[0].replace(/ Iterator] {$/," Entries] {"),Qt(e,r,o,Fr)):Xt(e,r,o,Tr)}function po(t,e,n){return[""]}function Z(t,e,n,r,o,i,s=e){let a,l,f=" ";if(i=i||me(e,r)||{value:e[r],enumerable:!0},i.value!==void 0){let c=t.compact!==!0||o!==J?2:3;t.indentationLvl+=c,l=A(t,i.value,n),c===3&&t.breakLength<it(l,t.colors)&&(f=`
${" ".repeat(t.indentationLvl)}`),t.indentationLvl-=c}else if(i.get!==void 0){let c=i.set!==void 0?"Getter/Setter":"Getter",p=t.stylize,y="special";if(t.getters&&(t.getters===!0||t.getters==="get"&&i.set===void 0||t.getters==="set"&&i.set!==void 0))try{let h=qe(i.get,s);if(t.indentationLvl+=2,h===null)l=`${p(`[${c}:`,y)} ${p("null","null")}${p("]",y)}`;else if(typeof h=="object")l=`${p(`[${c}]`,y)} ${A(t,h,n)}`;else{let E=at(p,h,t);l=`${p(`[${c}:`,y)} ${E}${p("]",y)}`}t.indentationLvl-=2}catch(h){let E=`<Inspection threw (${h.message})>`;l=`${p(`[${c}:`,y)} ${E}${p("]",y)}`}else l=t.stylize(`[${c}]`,y)}else i.set!==void 0?l=t.stylize("[Setter]","special"):l=t.stylize("undefined","undefined");if(o===ot)return l;if(typeof r=="symbol"){let c=ve(ze,Bt(r),Ue);a=`[${t.stylize(c,"symbol")}]`}else r==="__proto__"?a="['__proto__']":i.enumerable===!1?a=`[${ve(ze,r,Ue)}]`:nt(Ir,r)!==null?a=t.stylize(r,"name"):a=t.stylize(Ke(r),"string");return`${a}:${f}${l}`}function At(t,e,n,r){let o=e.length+n;if(o+e.length>t.breakLength)return!1;for(let i=0;i<e.length;i++)if(t.colors?o+=Dt(e[i]).length:o+=e[i].length,o>t.breakLength)return!1;return r===""||!te(r,`
`)}function lt(t,e,n,r,o,i,s){if(t.compact!==!0){if(typeof t.compact=="number"&&t.compact>=1){let c=e.length;if(o===ae&&c>6&&(e=no(t,e,s)),t.currentDepth-i<t.compact&&c===e.length){let p=e.length+t.indentationLvl+r[0].length+n.length+10;if(At(t,e,p,n)){let y=ne(e,", ");if(!y.includes(`
`))return`${n?`${n} `:""}${r[0]} ${y} ${r[1]}`}}}let f=`
${mt(" ",t.indentationLvl)}`;return`${n?`${n} `:""}${r[0]}${f}  ${ne(e,`,${f}  `)}${f}${r[1]}`}if(At(t,e,0,n))return`${r[0]}${n?` ${n}`:""} ${ne(e,", ")} `+r[1];let a=mt(" ",t.indentationLvl),l=n===""&&r[0].length===1?" ":`${n?` ${n}`:""}
${a}  `;return`${r[0]}${l}${ne(e,`,
${a}  `)} ${r[1]}`}it=function(e,n=!0){let r=0;n&&(e=ho(e)),e=Vn(e,"NFC");for(let o of new Dn(e)){let i=Kn(o,0);go(i)?r+=2:yo(i)||r++}return r};var go=t=>t>=4352&&(t<=4447||t===9001||t===9002||t>=11904&&t<=12871&&t!==12351||t>=12880&&t<=19903||t>=19968&&t<=42182||t>=43360&&t<=43388||t>=44032&&t<=55203||t>=63744&&t<=64255||t>=65040&&t<=65049||t>=65072&&t<=65131||t>=65281&&t<=65376||t>=65504&&t<=65510||t>=110592&&t<=110593||t>=127488&&t<=127569||t>=127744&&t<=128591||t>=131072&&t<=262141),yo=t=>t<=31||t>=127&&t<=159||t>=768&&t<=879||t>=8203&&t<=8207||t>=8400&&t<=8447||t>=65024&&t<=65039||t>=65056&&t<=65071||t>=917760&&t<=917999;function ho(t){return $r(t,"str"),t.replace(jr,"")}var en=O;var tn={length:!0,name:!0,arguments:!0,caller:!0,prototype:!0};var D=___altvEsbuild_altvInject_alt___,nn=___altvEsbuild_altvInject_altShared___,ct=class{log=new x("shared");resourceStopEvent=()=>{this.log.debug("resourceStop");for(let e of this.metaKeys)D.deleteMeta(e)};origAltOn;origAltOnce;origAltOff;origAltSetMeta;eventHandlers={local:{},remote:{}};metaKeys=new Set;baseObjects=new Set;hookedAltEvents={};eventHandlersWrappers=new Map;constructor(e){e.dev.enabled&&(this.origAltOn=this.hookAltEventAdd("local","on",1),this.origAltOnce=this.hookAltEventAdd("local","once",1,!0),this.origAltOff=this.hookAltEventRemove("local","off",1),this.hookAlt("getEventListeners",(n,r)=>typeof r=="string"?[...this.eventHandlers.local[r]??[]]:n(r),1),this.hookAlt("getRemoteEventListeners",(n,r)=>typeof r=="string"?[...this.eventHandlers.remote[r]??[]]:n(r),1),this.origAltSetMeta=this.hookAlt("setMeta",(n,r,o)=>{this.metaKeys.add(r),n(r,o)},2),this.origAltOn("resourceStop",this.resourceStopEvent),e.enhancedAltLog&&this.hookAltLogging())}hookAlt(e,n,r){let o=D[e];if(o==null)throw new Error(`[hookAlt] original property is not defined: ${e}`);if(Object.hasOwn(o,"___hookAlt"))throw new Error(`[hookAlt] already hooked property: ${e}`);return n.prototype||(n=n.bind(null,o)),Object.defineProperty(n,"___hookAlt",{enumerable:!1,configurable:!1,writable:!1,value:!0}),D[e]=(...i)=>{if(i.length<r)throw new Error(`${r} arguments expected`);return n(...i)},nn[e]!=null&&(nn[e]=n),o}hookAltEventAdd(e,n,r,o=!1){return this.hookAlt(n,(i,s,a)=>{if(!(typeof s=="string"||typeof s=="function"))throw new Error("Expected a string or function as first argument");if(typeof s=="function"){i(s);return}if(typeof a!="function")throw new Error("Expected a function as second argument");let l=this.eventHandlers[e];(l[s]??(l[s]=new Set)).add(a);let f=(...p)=>{if(o&&(l[s]?.delete(a),this.eventHandlersWrappers.get(s)?.delete(a)),this.hookedAltEvents[s]){this.log.debug("skip calling user handler of hooked alt event:",s);return}try{let y=a(...p);return y instanceof Promise?y.catch(h=>{throw this.logEventException(s,h),h}):y}catch(y){throw this.logEventException(s,y),y}},c=this.eventHandlersWrappers.get(s)??new Map;this.eventHandlersWrappers.set(s,c),c.set(a,f),i(s,f)},r)}hookAltEventRemove(e,n,r){return this.hookAlt(n,(o,i,s)=>{if(this.log.debug(`hooked alt.${n} called args:`,i,typeof s),!(typeof i=="string"||typeof i=="function"))throw new Error("Expected a string or function as first argument");if(typeof i=="function"){o(i);return}if(typeof s!="function")throw new Error("Expected a function as second argument");let a=this.eventHandlersWrappers.get(i);if(!a){this.log.debug(`alt.${n} called but event handlers are not registered for event: ${i}`);return}let l=a.get(s);if(!l){this.log.debug(`alt.${n} called but event handler is not registered for event: ${i}`);return}this.eventHandlers[e][i]?.delete(s),a?.delete(s),o(i,l)},r)}hookAltEvent(e,n){this.hookedAltEvents[e]=!0,this.log.debug("hooked alt event:",e);let r=this.origAltOn??D.on;this.log.debug("hookAltEvent altOn:",r),r(e,(...o)=>{this.log.debug("received hooked alt event:",e);let i=s=>{if(s===!1){this.log.debug("hooked altv event:",e,"was canceled");return}this.log.debug("hooked altv event:",e,"calling with args:",s),this.emitAltEvent(e,...s)};try{let s=n(...o);s instanceof Promise?s.then(a=>{i(a)}).catch(a=>{this.log.error(a)}):i(s)}catch(s){this.log.error("hook of altv event:",e,"error:",s)}})}emitAltEvent(e,...n){let r=this.eventHandlers.local[e];if(!r){this.log.debug("callAltEvent:",e,"no handlers");return}r.forEach(async o=>{try{await o(...n)}catch(i){this.logEventException(e,i)}})}setPlayerObjectPrototype(e,n=D.Player){Object.setPrototypeOf(e,n.prototype)}generateEventName(e){return`___${I}:${e}___`}wrapBaseObjectChildClass(e){let n=e.prototype,r=Symbol("originalDestroy"),o=this.baseObjects,i=this.log;n[r]=n.destroy,n.destroy=function(){try{o.delete(this),this[r]()}catch(a){throw i.error(`failed to destroy alt.${e.name} error:`),a}};let s=function(...a){try{let l=new e(...a);return o.add(l),Object.setPrototypeOf(l,this.__proto__),l}catch(l){throw i.error(`failed to create alt.${e.name} error:`),l}};s.prototype=e.prototype,Object.defineProperty(s,"name",{value:e.name});try{let a=Object.keys(e);for(let l of a)if(!tn[l])try{let{value:f,set:c}=Object.getOwnPropertyDescriptor(e,l);typeof f=="function"?s[l]=e[l]:Object.defineProperty(s,l,{get:()=>e[l],set:c?.bind(e)})}catch(f){this.log.error(`detected broken alt.${e.name} static property: ${l}. 
`,f?.stack??f)}}catch(a){this.log.error(a.stack??a)}return s}destroyBaseObjects(){this.log.debug("destroyBaseObjects count:",this.baseObjects.size);for(let e of this.baseObjects)e.destroy()}onResourceStop(e){this.origAltOn("resourceStop",e)}defineMetaSetter(e,n,r){return function(o,i){if(arguments.length<2)throw new Error("2 arguments expected");this[n](o,i),this[r]??={},this[r][o]=i}}hookAltLogging(){let e=(r,...o)=>{r(...o.map(i=>typeof i=="string"?i:en(i,{colors:!0})))},n=this.hookAlt("log",e,0);D.isClient&&(console.log=e.bind(null,n)),D.logDebug&&this.hookAlt("logDebug",e,0)}logEventException(e,n){D.logError(`Uncaught exception in event listener of event "${e}":
`,n?.stack??n)}},d=new ct(___altvEsbuild_altvInject_pluginOptions___);var V={restartCommand:d.generateEventName("restartCommand"),clientReady:d.generateEventName("clientReady")},fe={playerConnect:d.generateEventName("playerConnect"),connectionComplete:d.generateEventName("connectionComplete")};var P=___altvEsbuild_altvInject_alt___,$;P.isClient&&($=___altvEsbuild_altvInject_native___);var Ae=class{origAltOnServer;log=new x("client");clearPlayerMeta;onResourceStop=()=>{this.clearGame(),d.destroyBaseObjects(),this.clearPlayerMeta?.()};constructor(e){let{bugFixes:n,dev:r}=e;n.webViewFlickering&&this.initWebViewFlickeringBugFix(),r.enabled&&(this.origAltOnServer=d.hookAltEventAdd("remote","onServer",1),d.hookAltEventAdd("remote","onceServer",1,!0),d.hookAltEventRemove("remote","offServer",1),n.playerPrototype&&this.initPlayerPrototypeTempFix(),r.restartCommand&&this.initRestartConsoleCommand(e),r.disconnectEvent&&this.initDisconnectEvent(),r.connectionCompleteEvent&&(this.log.debug("dev.connectionCompleteEvent:",r.connectionCompleteEvent),this.initConnectionCompleteEvent()),r.clientServerInstanceValidation&&this.initClientServerInstanceValidation(),this.initClientReady(e),this.clearPlayerMeta=this.initPlayerMetaCleanup(),d.onResourceStop(this.onResourceStop))}initPlayerPrototypeTempFix(){P.nextTick(()=>{for(let e of P.Player.all)e.valid&&(e!==P.Player.local?d.setPlayerObjectPrototype(e):(this.log.debug("set local player prototype"),d.setPlayerObjectPrototype(e,P.LocalPlayer)))}),this.origAltOnServer(fe.playerConnect,e=>{d.setPlayerObjectPrototype(e)})}initRestartConsoleCommand(e){let n=e.dev.restartCommand===!0?"res":e.dev.restartCommand;this.log.debug("initRestartConsoleCommand command:",n),d.origAltOn("consoleCommand",r=>{r===n&&(this.log.info("~gl~restarting resource"),P.emitServerRaw(V.restartCommand))})}initWebViewFlickeringBugFix(){P.everyTick(()=>$.drawRect(0,0,0,0,0,0,0,0,!1))}initDisconnectEvent(){this.log.debug("initDisconnectEvent"),d.origAltOn("resourceStop",()=>d.emitAltEvent("disconnect"))}initConnectionCompleteEvent(){this.log.debug("initConnectionCompleteEvent");let e=!1;d.origAltOn("connectionComplete",()=>{e=!0}),this.origAltOnServer(fe.connectionComplete,()=>{this.log.debug("received connectionComplete"),!e&&d.emitAltEvent("connectionComplete")})}initClientReady(e){e.dev.clientServerInstanceValidation||P.emitServerRaw(V.clientReady)}initClientServerInstanceValidation(){P.nextTick(()=>{let e=typeof ___altvEsbuild_altvInject_instanceId___<"u"?___altvEsbuild_altvInject_instanceId___:"";this.log.debug("initClientServerInstanceValidation",{instanceId:e}),P.emitServerRaw(V.clientReady,e)})}clearGame(){let e=P.Player.local;$.freezeEntityPosition(e,!1),$.setEntityVisible(e,!0,!1),$.doScreenFadeIn(0),$.triggerScreenblurFadeOut(0),$.stopAudioScenes(),$.newLoadSceneStop(),$.destroyAllCams(!1),$.animpostfxStopAll(),$.setCamDeathFailEffectState(0),$.displayHud(!0),$.displayRadar(!0),P.FocusData.clearFocus(),$.setFrontendActive(!1),P.setCamFrozen(!1),$.setBigmapActive(!1,!1)}initPlayerMetaCleanup(){let e=P.Player.prototype,n=e,r=Symbol("metaStoreKey"),o=Symbol("originalSetMeta");return n[o]=e.setMeta,e.setMeta=d.defineMetaSetter(n,o,r),()=>{for(let i of P.Player.all){if(!i?.valid)continue;let s=i;for(let a in s[r])i.deleteMeta(a)}}}};var Pe=class{_promise;_resolve;_reject;constructor(){this._promise=new Promise((e,n)=>{this._resolve=e,this._reject=n})}get promise(){return this._promise}resolve(e){this._resolve(e)}reject(e){this._reject(e instanceof Error?e:new Error(e))}};var $e=class{constructor(e,n,r,o,i){this.name=e;this._net=n;this.port=r;this.host=o;this.connectHandler=i;this._socket=this.connect()}logDebug=()=>{};onError=e=>{(e?.code==="ECONNRESET"||e?.code==="ECONNREFUSED")&&(this.logDebug(`disconnected from server, trying reconnecting in ${$e.RECONNECT_MS}ms...`),setTimeout(()=>this.connect(),$e.RECONNECT_MS))};onConnect=e=>{this.connectHandler(e)};_socket;get socket(){return this._socket}connect(){this._socket&&this._socket.destroy();let e=this._net.connect(this.port,this.host===""?void 0:this.host);return e.on("connect",this.onConnect.bind(this,e)),e.on("error",this.onError),e}},de=$e;G(de,"RECONNECT_MS",500);var ft=`${I}:instanceId`;var rn=t=>"___altvEsbuild_"+t.replace(/[-/\\ @.:]/g,"_x_")+"___";var m=___altvEsbuild_altvInject_alt___,on,ke,sn,an;m.isServer&&(on=await(async()=>await import("net"))(),ke=await(async()=>await import("path"))(),sn=await(async()=>await import("fs"))(),an=await(async()=>await import("crypto"))());var Re=class{constructor(e){this.options=e;let{dev:n,bugFixes:r}=e;if(n.enabled){this.origAltOnClient=d.hookAltEventAdd("remote","onClient",1),d.hookAltEventAdd("remote","onceClient",1,!0),d.hookAltEventRemove("remote","offClient",1),d.hookAlt("setSyncedMeta",(s,a,l)=>{this.syncedMetaKeys.add(a),s(a,l)},2),this.hookBaseObjects();let o=this.hookAltPlayer(),i=()=>{};if(n.playersReconnect){this.initPlayersReconnect(e);let s=new m.Vector3((m.getServerConfig().mapBoundsMaxX??1e5)+2e3);this.log.debug("despawnPlayers streamOutPos:",s),i=this.despawnPlayers.bind(this,s)}d.onResourceStop(this.onResourceStop.bind(this,o,i)),this.log.debug("dev.hotReload:",n.hotReload),n.hotReload&&(this.log.debug("init socketConnect"),this.socketConnect=new de("server-inject",on,this.options.dev.hotReloadServerPort,this.options.dev.hotReloadServerHost,s=>{this.socket=s,this.eventManager=this.initEventManager(s),this.onConnect(),this.connectedAgain=!0})),n.enhancedRestartCommand?this.initEnhancedRestartCommand(e):n.restartCommand&&this.initRestartConsoleCommand(e),this.initClientReady(),r.playerPrototype&&this.initPlayerPrototypeTempFix(),n.serverStartedEvent&&this.initServerStartedEvent()}}events={buildStart:e=>{this.log.debug(`[buildStart] ms: ${new Date().getMilliseconds()} mode:`,e),this.onBuildStart(e)},buildEnd:(e,n)=>{if(this.log.debug("[buildEnd] received:",e),n&&(this.log.debug("received cached buildEnd -> emulate buildStart first"),this.onBuildStart(e)),!this.buildsInProgress.delete(e)){this.log.debug(`received unknown buildEnd: ${e}, do nothing`);return}if(this.buildsInProgress.size){this.log.debug("remaining builds in progress:",this.buildsInProgress.size),this.waitingForBuildEnd===e&&(this.waitingForBuildEnd=this.flipMode(e));return}if(this.waitingForBuildEnd!==e){this.log.debug("received not what we waiting for");return}this.log.debug("no builds in progress -> restart"),this.restartResource()},clientConnect:()=>{this.log.debug("clientConnect"),this.clientConnected=!0},clientDisconnect:()=>{this.log.debug("clientDisconnect"),this.clientConnected=!1}};onConnect=()=>{if(this.connectedAgain){this.restartResource();return}this.log.debug("net socket connected, sending connect server event"),this.sendEvent("connect","server")};onSocketError=(e,n)=>{this.log.error("[events]",e,n)};onResourceStop=(e,n)=>{this.log.debug("resourceStop");for(let r of this.syncedMetaKeys)this.log.debug("deleting synced meta key:",r),m.deleteSyncedMeta(r);e(),n(),d.destroyBaseObjects()};socket;eventManager;log=new x("server");clientConnected=!1;anotherBuildStartTimeout=null;waitingForBuildEnd=null;restartInProgress=!1;connectedAgain=!1;instanceId=this.getCurrentInstanceId();buildsInProgress=new Set;syncedMetaKeys=new Set;origAltOnClient;socketConnect;playerReadyEvents=new Map;sendEvent(e,...n){if(!this.eventManager){this.log.error("[sendEvent] no event manager");return}this.eventManager?.send(e,...n)}restartResource(){if(this.restartInProgress){this.log.error("resource restart already in progress");return}this.restartInProgress=!0,this.clearCurrentBuild(),this.options.dev.clientServerInstanceValidation&&this.appendInstanceIdToClientBundle();let e=this.getFullResourceName();this.log.info(`restarting resource ${e}...`),m.restartResource(e)}clearCurrentBuild(){this.waitingForBuildEnd=null,this.buildsInProgress.clear()}flipMode(e){return e==="client"?"server":"client"}initEventManager(e){return new ue(e,this.events,this.onSocketError)}hookAltPlayer(){let e=m.Player.prototype,n=Symbol("metaStoreKey"),r=Symbol("syncedMetaStoreKey"),o=Symbol("streamSyncedMetaStoreKey"),i=Symbol("localMetaStoreKey"),s=Symbol("originalSetMeta"),a=Symbol("originalSetSyncedMeta"),l=Symbol("originalSetStreamSyncedMeta"),f=Symbol("originalSetLocalMeta"),c=e;return c[s]=e.setMeta,c[a]=e.setSyncedMeta,c[l]=e.setStreamSyncedMeta,c[f]=e.setLocalMeta,e.setMeta=d.defineMetaSetter(c,s,n),e.setSyncedMeta=d.defineMetaSetter(c,a,r),e.setStreamSyncedMeta=d.defineMetaSetter(c,l,o),e.setLocalMeta=d.defineMetaSetter(c,f,i),()=>{for(let p of m.Player.all){if(!p?.valid)continue;let y=p;for(let h in y[n])p.deleteMeta(h);for(let h in y[r])p.deleteSyncedMeta(h);for(let h in y[o])p.deleteStreamSyncedMeta(h);for(let h in y[i])p.deleteLocalMeta(h)}}}initPlayersReconnect({dev:{playersReconnectDelay:e}}){let n=`${I}:resourceRestarted`;if(this.log.debug("_alt.getMeta(resourceRestartedKey):",m.getMeta(n)),!m.getMeta(n)){this.log.debug("set resource restarted"),d.origAltSetMeta(n,!0);return}let r=m.Player.all;if(!r.length){this.log.debug("no players to reconnect");return}for(let o of r)o.valid&&this.initPlayerReadyEvent(o);this.log.info(`start a timer for ~cl~${e}~w~ ms to reconnect players (${r.length})`),setTimeout(()=>{this.log.debug("reconnecting players count:",r.length);for(let o of r)o.valid&&(o.dimension=m.defaultDimension,o.streamed=!0,o.collision=!0,o.invincible=!1,o.visible=!0,o.frozen=!1,this.waitForPlayerReadyEvent(o).then(i=>{if(!i){this.log.debug("waitForPlayerReadyEvent promise resolved false, player disconnected");return}this.log.debug("waitForPlayerReadyEvent success player:",o.name,o.id),d.emitAltEvent("playerConnect",o)}).catch(i=>{this.log.error(i.stack)}))},e)}despawnPlayers(e){this.log.debug("despawn players");for(let n of m.Player.all)n.valid&&(n.removeAllWeapons(),n.clearBloodDamage(),n.detach(),n.despawn(),n.visible=!1,this.options.dev.playersReconnectResetPos&&(n.pos=e))}initPlayerPrototypeTempFix(){m.nextTick(()=>{for(let e of m.Player.all)e.valid&&d.setPlayerObjectPrototype(e)}),d.origAltOn("playerConnect",e=>{d.setPlayerObjectPrototype(e)})}initRestartConsoleCommand(e){let n=e.dev.restartCommand===!0?"res":e.dev.restartCommand,r=()=>{this.restartResource()};d.origAltOn("consoleCommand",o=>{o===n&&r()}),this.origAltOnClient(V.restartCommand,()=>{r()})}initEnhancedRestartCommand({dev:{enhancedRestartCommand:e}}){let n=e===!0?"res":e;if(m.hasResource(Q))this.log.debug("control resource already started",Q);else{this.log.debug("control resource is not started",Q);let r=this.resolveRelativePathOfResourceControl();this.log.debug({resourceControlPath:r}),m.nextTick(()=>{d.origAltOnce(d.generateEventName("resourceControlReady"),()=>{m.emit(d.generateEventName("resourceControlInit"),this.getFullResourceName(),n)}),m.startResource(r)})}}initClientReady(){this.log.debug("initClientReady"),this.origAltOnClient(V.clientReady,(e,n)=>{if(this.log.debug("received clientReady player:",e.name,e.id,{instanceId:n}),this.options.dev.clientServerInstanceValidation){if(this.instanceId!==""&&this.instanceId!==n){this.log.debug("this.instanceId !== InstanceId");return}this.log.debug("clientServerInstanceValidation ~gl~success")}this.options.dev.connectionCompleteEvent&&e.emitRaw(fe.connectionComplete);let r=this.playerReadyEvents.get(e);if(!r){this.log.debug("cant get ready event, skip");return}r.resolve(!0)})}hookBaseObjects(){for(let e in m){let n=e,r=m[n];if(this.isAltVirtualEntityGroupClass(r)||!this.isBaseObjectClass(r))continue;let o=!1;try{new r,o=!0}catch(i){i?.message?.includes("abstract")&&(o=!0)}o||(m[n]=d.wrapBaseObjectChildClass(r),this.log.debug("wrapped base object class:",r.name))}}isAltVirtualEntityGroupClass(e){return e.name==="VirtualEntityGroup"}isBaseObjectClass(e){return e.prototype instanceof m.BaseObject&&e!==m.Player}onBuildStart(e){let n=this.flipMode(e);if(this.buildsInProgress.add(e),this.buildsInProgress.size===1&&this.waitingForBuildEnd===n&&this.buildsInProgress.has(n)){this.log.debug(`waiting currently for first build, change waitingForBuildEnd to: ${e}`),this.waitingForBuildEnd=e;return}if(this.anotherBuildStartTimeout){this.log.debug("[buildStart] received another build:",e,"clear timeout"),clearTimeout(this.anotherBuildStartTimeout),this.anotherBuildStartTimeout=null;return}if(e==="server"&&!this.clientConnected){this.log.debug("[buildStart] client is not connected, skip waiting for another build"),this.waitingForBuildEnd=e;return}this.waitingForBuildEnd=n,this.log.debug(`[buildStart] waiting for another build: ${this.waitingForBuildEnd} to start...`),this.anotherBuildStartTimeout=setTimeout(()=>{this.anotherBuildStartTimeout=null,this.log.debug(`another build didnt started after ${Re.MAX_ANOTHER_BUILD_START_MS}ms`),this.buildsInProgress.has(e)?(this.log.debug(`waiting for build: ${e} to end now...`),this.waitingForBuildEnd=e):(this.log.debug(`first build: ${e} ended -> restart`),this.restartResource())},Re.MAX_ANOTHER_BUILD_START_MS)}initPlayerReadyEvent(e){let n=new Pe;this.playerReadyEvents.set(e,n);let r=o=>{o===e&&(this.playerReadyEvents.delete(e),n.resolve(!1))};n.promise.finally(()=>{d.origAltOff("playerDisconnect",r)}),d.origAltOn("playerDisconnect",r),this.origAltOnClient(d.generateEventName("playerReady"),r)}async waitForPlayerReadyEvent(e){let n=this.playerReadyEvents.get(e);return n?await n.promise:(this.log.warn("waitForPlayerReadyEvent unknown player:",e.name,e.id),!1)}getFullResourceName(){let{path:e}=m.Resource.current,n=this.getResourcesDir();return e.slice(n.length).replaceAll("\\","/")}getResourcesDir(){return`${m.rootDir}\\resources\\`}initServerStartedEvent(){this.log.debug("initServerStartedEvent");let e=new m.Utils.Timeout(()=>{e=null,this.log.debug("emitting serverStarted from timer"),d.emitAltEvent("serverStarted")},500);d.hookAltEvent("serverStarted",(...n)=>e?(e?.destroy(),e=null,this.log.debug("emitting serverStarted from original"),n):(this.log.error("original serverStarted was called, but timer is already null"),!1))}nextInstanceId(){return this.instanceId=an.randomUUID(),d.origAltSetMeta(ft,this.instanceId.toString()),this.instanceId}getCurrentInstanceId(){return m.getMeta(ft)??""}appendInstanceIdToClientBundle(){let e=m.Resource.current,n=e.config["client-main"];if(n==null)throw new Error("[clientServerInstanceValidation] Failed to get client-main from resource config");let r=ke.join(e.path,n),o=this.nextInstanceId();this.log.debug({instanceId:o});let i=rn("altvInject_instanceId");sn.writeFileSync(r,`

globalThis.${i} = "${o}"`,{flag:"a"})}resolveRelativePathOfResourceControl(){let e=ke.relative(this.getResourcesDir(),___altvEsbuild_altvInject_pluginDistDir___);return e=ke.join(e,Q),e=e.replaceAll("\\","/"),e}},Y=Re;G(Y,"MAX_ANOTHER_BUILD_START_MS",500),G(Y,"RECONNECT_MS",500);var dt=___altvEsbuild_altvInject_pluginOptions___;dt.mode==="client"?new Ae(dt):new Y(dt);
})().catch(e => ___altvEsbuild_altvInject_altShared___.logError("[altv-esbuild] banner wrapper error:", e?.stack ?? e?.message ?? e));
// ------------------- altv-esbuild banner -------------------

var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));

// altv-esbuild:external-handling-alt-client:alt-client
var require_alt_client = __commonJS({
  "altv-esbuild:external-handling-alt-client:alt-client"(exports, module) {
    module.exports = ___altvEsbuild_altvInject_alt___;
  }
});

// source-map/lib/base64.js
var require_base64 = __commonJS({
  "source-map/lib/base64.js"(exports) {
    var intToCharMap = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/".split("");
    exports.encode = function(number) {
      if (0 <= number && number < intToCharMap.length) {
        return intToCharMap[number];
      }
      throw new TypeError("Must be between 0 and 63: " + number);
    };
  }
});

// source-map/lib/base64-vlq.js
var require_base64_vlq = __commonJS({
  "source-map/lib/base64-vlq.js"(exports) {
    var base64 = require_base64();
    var VLQ_BASE_SHIFT = 5;
    var VLQ_BASE = 1 << VLQ_BASE_SHIFT;
    var VLQ_BASE_MASK = VLQ_BASE - 1;
    var VLQ_CONTINUATION_BIT = VLQ_BASE;
    function toVLQSigned(aValue) {
      return aValue < 0 ? (-aValue << 1) + 1 : (aValue << 1) + 0;
    }
    exports.encode = function base64VLQ_encode(aValue) {
      let encoded = "";
      let digit;
      let vlq = toVLQSigned(aValue);
      do {
        digit = vlq & VLQ_BASE_MASK;
        vlq >>>= VLQ_BASE_SHIFT;
        if (vlq > 0) {
          digit |= VLQ_CONTINUATION_BIT;
        }
        encoded += base64.encode(digit);
      } while (vlq > 0);
      return encoded;
    };
  }
});

// source-map/lib/util.js
var require_util = __commonJS({
  "source-map/lib/util.js"(exports) {
    function getArg(aArgs, aName, aDefaultValue) {
      if (aName in aArgs) {
        return aArgs[aName];
      } else if (arguments.length === 3) {
        return aDefaultValue;
      }
      throw new Error('"' + aName + '" is a required argument.');
    }
    exports.getArg = getArg;
    var urlRegexp = /^(?:([\w+\-.]+):)?\/\/(?:(\w+:\w+)@)?([\w.-]*)(?::(\d+))?(.*)$/;
    var dataUrlRegexp = /^data:.+\,.+$/;
    function urlParse(aUrl) {
      const match = aUrl.match(urlRegexp);
      if (!match) {
        return null;
      }
      return {
        scheme: match[1],
        auth: match[2],
        host: match[3],
        port: match[4],
        path: match[5]
      };
    }
    exports.urlParse = urlParse;
    function urlGenerate(aParsedUrl) {
      let url = "";
      if (aParsedUrl.scheme) {
        url += aParsedUrl.scheme + ":";
      }
      url += "//";
      if (aParsedUrl.auth) {
        url += aParsedUrl.auth + "@";
      }
      if (aParsedUrl.host) {
        url += aParsedUrl.host;
      }
      if (aParsedUrl.port) {
        url += ":" + aParsedUrl.port;
      }
      if (aParsedUrl.path) {
        url += aParsedUrl.path;
      }
      return url;
    }
    exports.urlGenerate = urlGenerate;
    var MAX_CACHED_INPUTS = 32;
    function lruMemoize(f) {
      const cache = [];
      return function(input) {
        for (let i = 0; i < cache.length; i++) {
          if (cache[i].input === input) {
            const temp = cache[0];
            cache[0] = cache[i];
            cache[i] = temp;
            return cache[0].result;
          }
        }
        const result = f(input);
        cache.unshift({
          input,
          result
        });
        if (cache.length > MAX_CACHED_INPUTS) {
          cache.pop();
        }
        return result;
      };
    }
    var normalize = lruMemoize(function normalize2(aPath) {
      let path = aPath;
      const url = urlParse(aPath);
      if (url) {
        if (!url.path) {
          return aPath;
        }
        path = url.path;
      }
      const isAbsolute = exports.isAbsolute(path);
      const parts = [];
      let start = 0;
      let i = 0;
      while (true) {
        start = i;
        i = path.indexOf("/", start);
        if (i === -1) {
          parts.push(path.slice(start));
          break;
        } else {
          parts.push(path.slice(start, i));
          while (i < path.length && path[i] === "/") {
            i++;
          }
        }
      }
      let up = 0;
      for (i = parts.length - 1; i >= 0; i--) {
        const part = parts[i];
        if (part === ".") {
          parts.splice(i, 1);
        } else if (part === "..") {
          up++;
        } else if (up > 0) {
          if (part === "") {
            parts.splice(i + 1, up);
            up = 0;
          } else {
            parts.splice(i, 2);
            up--;
          }
        }
      }
      path = parts.join("/");
      if (path === "") {
        path = isAbsolute ? "/" : ".";
      }
      if (url) {
        url.path = path;
        return urlGenerate(url);
      }
      return path;
    });
    exports.normalize = normalize;
    function join(aRoot, aPath) {
      if (aRoot === "") {
        aRoot = ".";
      }
      if (aPath === "") {
        aPath = ".";
      }
      const aPathUrl = urlParse(aPath);
      const aRootUrl = urlParse(aRoot);
      if (aRootUrl) {
        aRoot = aRootUrl.path || "/";
      }
      if (aPathUrl && !aPathUrl.scheme) {
        if (aRootUrl) {
          aPathUrl.scheme = aRootUrl.scheme;
        }
        return urlGenerate(aPathUrl);
      }
      if (aPathUrl || aPath.match(dataUrlRegexp)) {
        return aPath;
      }
      if (aRootUrl && !aRootUrl.host && !aRootUrl.path) {
        aRootUrl.host = aPath;
        return urlGenerate(aRootUrl);
      }
      const joined = aPath.charAt(0) === "/" ? aPath : normalize(aRoot.replace(/\/+$/, "") + "/" + aPath);
      if (aRootUrl) {
        aRootUrl.path = joined;
        return urlGenerate(aRootUrl);
      }
      return joined;
    }
    exports.join = join;
    exports.isAbsolute = function(aPath) {
      return aPath.charAt(0) === "/" || urlRegexp.test(aPath);
    };
    function relative(aRoot, aPath) {
      if (aRoot === "") {
        aRoot = ".";
      }
      aRoot = aRoot.replace(/\/$/, "");
      let level = 0;
      while (aPath.indexOf(aRoot + "/") !== 0) {
        const index = aRoot.lastIndexOf("/");
        if (index < 0) {
          return aPath;
        }
        aRoot = aRoot.slice(0, index);
        if (aRoot.match(/^([^\/]+:\/)?\/*$/)) {
          return aPath;
        }
        ++level;
      }
      return Array(level + 1).join("../") + aPath.substr(aRoot.length + 1);
    }
    exports.relative = relative;
    var supportsNullProto = function() {
      const obj = /* @__PURE__ */ Object.create(null);
      return !("__proto__" in obj);
    }();
    function identity(s) {
      return s;
    }
    function toSetString(aStr) {
      if (isProtoString(aStr)) {
        return "$" + aStr;
      }
      return aStr;
    }
    exports.toSetString = supportsNullProto ? identity : toSetString;
    function fromSetString(aStr) {
      if (isProtoString(aStr)) {
        return aStr.slice(1);
      }
      return aStr;
    }
    exports.fromSetString = supportsNullProto ? identity : fromSetString;
    function isProtoString(s) {
      if (!s) {
        return false;
      }
      const length = s.length;
      if (length < 9) {
        return false;
      }
      if (s.charCodeAt(length - 1) !== 95 || s.charCodeAt(length - 2) !== 95 || s.charCodeAt(length - 3) !== 111 || s.charCodeAt(length - 4) !== 116 || s.charCodeAt(length - 5) !== 111 || s.charCodeAt(length - 6) !== 114 || s.charCodeAt(length - 7) !== 112 || s.charCodeAt(length - 8) !== 95 || s.charCodeAt(length - 9) !== 95) {
        return false;
      }
      for (let i = length - 10; i >= 0; i--) {
        if (s.charCodeAt(i) !== 36) {
          return false;
        }
      }
      return true;
    }
    function compareByOriginalPositions(mappingA, mappingB, onlyCompareOriginal) {
      let cmp = strcmp(mappingA.source, mappingB.source);
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.originalLine - mappingB.originalLine;
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.originalColumn - mappingB.originalColumn;
      if (cmp !== 0 || onlyCompareOriginal) {
        return cmp;
      }
      cmp = mappingA.generatedColumn - mappingB.generatedColumn;
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.generatedLine - mappingB.generatedLine;
      if (cmp !== 0) {
        return cmp;
      }
      return strcmp(mappingA.name, mappingB.name);
    }
    exports.compareByOriginalPositions = compareByOriginalPositions;
    function compareByGeneratedPositionsDeflated(mappingA, mappingB, onlyCompareGenerated) {
      let cmp = mappingA.generatedLine - mappingB.generatedLine;
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.generatedColumn - mappingB.generatedColumn;
      if (cmp !== 0 || onlyCompareGenerated) {
        return cmp;
      }
      cmp = strcmp(mappingA.source, mappingB.source);
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.originalLine - mappingB.originalLine;
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.originalColumn - mappingB.originalColumn;
      if (cmp !== 0) {
        return cmp;
      }
      return strcmp(mappingA.name, mappingB.name);
    }
    exports.compareByGeneratedPositionsDeflated = compareByGeneratedPositionsDeflated;
    function strcmp(aStr1, aStr2) {
      if (aStr1 === aStr2) {
        return 0;
      }
      if (aStr1 === null) {
        return 1;
      }
      if (aStr2 === null) {
        return -1;
      }
      if (aStr1 > aStr2) {
        return 1;
      }
      return -1;
    }
    function compareByGeneratedPositionsInflated(mappingA, mappingB) {
      let cmp = mappingA.generatedLine - mappingB.generatedLine;
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.generatedColumn - mappingB.generatedColumn;
      if (cmp !== 0) {
        return cmp;
      }
      cmp = strcmp(mappingA.source, mappingB.source);
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.originalLine - mappingB.originalLine;
      if (cmp !== 0) {
        return cmp;
      }
      cmp = mappingA.originalColumn - mappingB.originalColumn;
      if (cmp !== 0) {
        return cmp;
      }
      return strcmp(mappingA.name, mappingB.name);
    }
    exports.compareByGeneratedPositionsInflated = compareByGeneratedPositionsInflated;
    function parseSourceMapInput(str) {
      return JSON.parse(str.replace(/^\)]}'[^\n]*\n/, ""));
    }
    exports.parseSourceMapInput = parseSourceMapInput;
    function computeSourceURL(sourceRoot, sourceURL, sourceMapURL) {
      sourceURL = sourceURL || "";
      if (sourceRoot) {
        if (sourceRoot[sourceRoot.length - 1] !== "/" && sourceURL[0] !== "/") {
          sourceRoot += "/";
        }
        sourceURL = sourceRoot + sourceURL;
      }
      if (sourceMapURL) {
        const parsed = urlParse(sourceMapURL);
        if (!parsed) {
          throw new Error("sourceMapURL could not be parsed");
        }
        if (parsed.path) {
          const index = parsed.path.lastIndexOf("/");
          if (index >= 0) {
            parsed.path = parsed.path.substring(0, index + 1);
          }
        }
        sourceURL = join(urlGenerate(parsed), sourceURL);
      }
      return normalize(sourceURL);
    }
    exports.computeSourceURL = computeSourceURL;
  }
});

// source-map/lib/array-set.js
var require_array_set = __commonJS({
  "source-map/lib/array-set.js"(exports) {
    var ArraySet = class {
      constructor() {
        this._array = [];
        this._set = /* @__PURE__ */ new Map();
      }
      static fromArray(aArray, aAllowDuplicates) {
        const set = new ArraySet();
        for (let i = 0, len = aArray.length; i < len; i++) {
          set.add(aArray[i], aAllowDuplicates);
        }
        return set;
      }
      size() {
        return this._set.size;
      }
      add(aStr, aAllowDuplicates) {
        const isDuplicate = this.has(aStr);
        const idx = this._array.length;
        if (!isDuplicate || aAllowDuplicates) {
          this._array.push(aStr);
        }
        if (!isDuplicate) {
          this._set.set(aStr, idx);
        }
      }
      has(aStr) {
        return this._set.has(aStr);
      }
      indexOf(aStr) {
        const idx = this._set.get(aStr);
        if (idx >= 0) {
          return idx;
        }
        throw new Error('"' + aStr + '" is not in the set.');
      }
      at(aIdx) {
        if (aIdx >= 0 && aIdx < this._array.length) {
          return this._array[aIdx];
        }
        throw new Error("No element indexed by " + aIdx);
      }
      toArray() {
        return this._array.slice();
      }
    };
    exports.ArraySet = ArraySet;
  }
});

// source-map/lib/mapping-list.js
var require_mapping_list = __commonJS({
  "source-map/lib/mapping-list.js"(exports) {
    var util = require_util();
    function generatedPositionAfter(mappingA, mappingB) {
      const lineA = mappingA.generatedLine;
      const lineB = mappingB.generatedLine;
      const columnA = mappingA.generatedColumn;
      const columnB = mappingB.generatedColumn;
      return lineB > lineA || lineB == lineA && columnB >= columnA || util.compareByGeneratedPositionsInflated(mappingA, mappingB) <= 0;
    }
    var MappingList = class {
      constructor() {
        this._array = [];
        this._sorted = true;
        this._last = { generatedLine: -1, generatedColumn: 0 };
      }
      unsortedForEach(aCallback, aThisArg) {
        this._array.forEach(aCallback, aThisArg);
      }
      add(aMapping) {
        if (generatedPositionAfter(this._last, aMapping)) {
          this._last = aMapping;
          this._array.push(aMapping);
        } else {
          this._sorted = false;
          this._array.push(aMapping);
        }
      }
      toArray() {
        if (!this._sorted) {
          this._array.sort(util.compareByGeneratedPositionsInflated);
          this._sorted = true;
        }
        return this._array;
      }
    };
    exports.MappingList = MappingList;
  }
});

// source-map/lib/source-map-generator.js
var require_source_map_generator = __commonJS({
  "source-map/lib/source-map-generator.js"(exports) {
    var base64VLQ = require_base64_vlq();
    var util = require_util();
    var ArraySet = require_array_set().ArraySet;
    var MappingList = require_mapping_list().MappingList;
    var SourceMapGenerator = class {
      constructor(aArgs) {
        if (!aArgs) {
          aArgs = {};
        }
        this._file = util.getArg(aArgs, "file", null);
        this._sourceRoot = util.getArg(aArgs, "sourceRoot", null);
        this._skipValidation = util.getArg(aArgs, "skipValidation", false);
        this._sources = new ArraySet();
        this._names = new ArraySet();
        this._mappings = new MappingList();
        this._sourcesContents = null;
      }
      static fromSourceMap(aSourceMapConsumer) {
        const sourceRoot = aSourceMapConsumer.sourceRoot;
        const generator = new SourceMapGenerator({
          file: aSourceMapConsumer.file,
          sourceRoot
        });
        aSourceMapConsumer.eachMapping(function(mapping) {
          const newMapping = {
            generated: {
              line: mapping.generatedLine,
              column: mapping.generatedColumn
            }
          };
          if (mapping.source != null) {
            newMapping.source = mapping.source;
            if (sourceRoot != null) {
              newMapping.source = util.relative(sourceRoot, newMapping.source);
            }
            newMapping.original = {
              line: mapping.originalLine,
              column: mapping.originalColumn
            };
            if (mapping.name != null) {
              newMapping.name = mapping.name;
            }
          }
          generator.addMapping(newMapping);
        });
        aSourceMapConsumer.sources.forEach(function(sourceFile) {
          let sourceRelative = sourceFile;
          if (sourceRoot !== null) {
            sourceRelative = util.relative(sourceRoot, sourceFile);
          }
          if (!generator._sources.has(sourceRelative)) {
            generator._sources.add(sourceRelative);
          }
          const content = aSourceMapConsumer.sourceContentFor(sourceFile);
          if (content != null) {
            generator.setSourceContent(sourceFile, content);
          }
        });
        return generator;
      }
      addMapping(aArgs) {
        const generated = util.getArg(aArgs, "generated");
        const original = util.getArg(aArgs, "original", null);
        let source = util.getArg(aArgs, "source", null);
        let name = util.getArg(aArgs, "name", null);
        if (!this._skipValidation) {
          this._validateMapping(generated, original, source, name);
        }
        if (source != null) {
          source = String(source);
          if (!this._sources.has(source)) {
            this._sources.add(source);
          }
        }
        if (name != null) {
          name = String(name);
          if (!this._names.has(name)) {
            this._names.add(name);
          }
        }
        this._mappings.add({
          generatedLine: generated.line,
          generatedColumn: generated.column,
          originalLine: original != null && original.line,
          originalColumn: original != null && original.column,
          source,
          name
        });
      }
      setSourceContent(aSourceFile, aSourceContent) {
        let source = aSourceFile;
        if (this._sourceRoot != null) {
          source = util.relative(this._sourceRoot, source);
        }
        if (aSourceContent != null) {
          if (!this._sourcesContents) {
            this._sourcesContents = /* @__PURE__ */ Object.create(null);
          }
          this._sourcesContents[util.toSetString(source)] = aSourceContent;
        } else if (this._sourcesContents) {
          delete this._sourcesContents[util.toSetString(source)];
          if (Object.keys(this._sourcesContents).length === 0) {
            this._sourcesContents = null;
          }
        }
      }
      applySourceMap(aSourceMapConsumer, aSourceFile, aSourceMapPath) {
        let sourceFile = aSourceFile;
        if (aSourceFile == null) {
          if (aSourceMapConsumer.file == null) {
            throw new Error(
              `SourceMapGenerator.prototype.applySourceMap requires either an explicit source file, or the source map's "file" property. Both were omitted.`
            );
          }
          sourceFile = aSourceMapConsumer.file;
        }
        const sourceRoot = this._sourceRoot;
        if (sourceRoot != null) {
          sourceFile = util.relative(sourceRoot, sourceFile);
        }
        const newSources = this._mappings.toArray().length > 0 ? new ArraySet() : this._sources;
        const newNames = new ArraySet();
        this._mappings.unsortedForEach(function(mapping) {
          if (mapping.source === sourceFile && mapping.originalLine != null) {
            const original = aSourceMapConsumer.originalPositionFor({
              line: mapping.originalLine,
              column: mapping.originalColumn
            });
            if (original.source != null) {
              mapping.source = original.source;
              if (aSourceMapPath != null) {
                mapping.source = util.join(aSourceMapPath, mapping.source);
              }
              if (sourceRoot != null) {
                mapping.source = util.relative(sourceRoot, mapping.source);
              }
              mapping.originalLine = original.line;
              mapping.originalColumn = original.column;
              if (original.name != null) {
                mapping.name = original.name;
              }
            }
          }
          const source = mapping.source;
          if (source != null && !newSources.has(source)) {
            newSources.add(source);
          }
          const name = mapping.name;
          if (name != null && !newNames.has(name)) {
            newNames.add(name);
          }
        }, this);
        this._sources = newSources;
        this._names = newNames;
        aSourceMapConsumer.sources.forEach(function(srcFile) {
          const content = aSourceMapConsumer.sourceContentFor(srcFile);
          if (content != null) {
            if (aSourceMapPath != null) {
              srcFile = util.join(aSourceMapPath, srcFile);
            }
            if (sourceRoot != null) {
              srcFile = util.relative(sourceRoot, srcFile);
            }
            this.setSourceContent(srcFile, content);
          }
        }, this);
      }
      _validateMapping(aGenerated, aOriginal, aSource, aName) {
        if (aOriginal && typeof aOriginal.line !== "number" && typeof aOriginal.column !== "number") {
          throw new Error(
            "original.line and original.column are not numbers -- you probably meant to omit the original mapping entirely and only map the generated position. If so, pass null for the original mapping instead of an object with empty or null values."
          );
        }
        if (aGenerated && "line" in aGenerated && "column" in aGenerated && aGenerated.line > 0 && aGenerated.column >= 0 && !aOriginal && !aSource && !aName) {
        } else if (aGenerated && "line" in aGenerated && "column" in aGenerated && aOriginal && "line" in aOriginal && "column" in aOriginal && aGenerated.line > 0 && aGenerated.column >= 0 && aOriginal.line > 0 && aOriginal.column >= 0 && aSource) {
        } else {
          throw new Error("Invalid mapping: " + JSON.stringify({
            generated: aGenerated,
            source: aSource,
            original: aOriginal,
            name: aName
          }));
        }
      }
      _serializeMappings() {
        let previousGeneratedColumn = 0;
        let previousGeneratedLine = 1;
        let previousOriginalColumn = 0;
        let previousOriginalLine = 0;
        let previousName = 0;
        let previousSource = 0;
        let result = "";
        let next;
        let mapping;
        let nameIdx;
        let sourceIdx;
        const mappings = this._mappings.toArray();
        for (let i = 0, len = mappings.length; i < len; i++) {
          mapping = mappings[i];
          next = "";
          if (mapping.generatedLine !== previousGeneratedLine) {
            previousGeneratedColumn = 0;
            while (mapping.generatedLine !== previousGeneratedLine) {
              next += ";";
              previousGeneratedLine++;
            }
          } else if (i > 0) {
            if (!util.compareByGeneratedPositionsInflated(mapping, mappings[i - 1])) {
              continue;
            }
            next += ",";
          }
          next += base64VLQ.encode(mapping.generatedColumn - previousGeneratedColumn);
          previousGeneratedColumn = mapping.generatedColumn;
          if (mapping.source != null) {
            sourceIdx = this._sources.indexOf(mapping.source);
            next += base64VLQ.encode(sourceIdx - previousSource);
            previousSource = sourceIdx;
            next += base64VLQ.encode(mapping.originalLine - 1 - previousOriginalLine);
            previousOriginalLine = mapping.originalLine - 1;
            next += base64VLQ.encode(mapping.originalColumn - previousOriginalColumn);
            previousOriginalColumn = mapping.originalColumn;
            if (mapping.name != null) {
              nameIdx = this._names.indexOf(mapping.name);
              next += base64VLQ.encode(nameIdx - previousName);
              previousName = nameIdx;
            }
          }
          result += next;
        }
        return result;
      }
      _generateSourcesContent(aSources, aSourceRoot) {
        return aSources.map(function(source) {
          if (!this._sourcesContents) {
            return null;
          }
          if (aSourceRoot != null) {
            source = util.relative(aSourceRoot, source);
          }
          const key = util.toSetString(source);
          return Object.prototype.hasOwnProperty.call(this._sourcesContents, key) ? this._sourcesContents[key] : null;
        }, this);
      }
      toJSON() {
        const map = {
          version: this._version,
          sources: this._sources.toArray(),
          names: this._names.toArray(),
          mappings: this._serializeMappings()
        };
        if (this._file != null) {
          map.file = this._file;
        }
        if (this._sourceRoot != null) {
          map.sourceRoot = this._sourceRoot;
        }
        if (this._sourcesContents) {
          map.sourcesContent = this._generateSourcesContent(map.sources, map.sourceRoot);
        }
        return map;
      }
      toString() {
        return JSON.stringify(this.toJSON());
      }
    };
    SourceMapGenerator.prototype._version = 3;
    exports.SourceMapGenerator = SourceMapGenerator;
  }
});

// source-map/lib/binary-search.js
var require_binary_search = __commonJS({
  "source-map/lib/binary-search.js"(exports) {
    exports.GREATEST_LOWER_BOUND = 1;
    exports.LEAST_UPPER_BOUND = 2;
    function recursiveSearch(aLow, aHigh, aNeedle, aHaystack, aCompare, aBias) {
      const mid = Math.floor((aHigh - aLow) / 2) + aLow;
      const cmp = aCompare(aNeedle, aHaystack[mid], true);
      if (cmp === 0) {
        return mid;
      } else if (cmp > 0) {
        if (aHigh - mid > 1) {
          return recursiveSearch(mid, aHigh, aNeedle, aHaystack, aCompare, aBias);
        }
        if (aBias == exports.LEAST_UPPER_BOUND) {
          return aHigh < aHaystack.length ? aHigh : -1;
        }
        return mid;
      }
      if (mid - aLow > 1) {
        return recursiveSearch(aLow, mid, aNeedle, aHaystack, aCompare, aBias);
      }
      if (aBias == exports.LEAST_UPPER_BOUND) {
        return mid;
      }
      return aLow < 0 ? -1 : aLow;
    }
    exports.search = function search(aNeedle, aHaystack, aCompare, aBias) {
      if (aHaystack.length === 0) {
        return -1;
      }
      let index = recursiveSearch(
        -1,
        aHaystack.length,
        aNeedle,
        aHaystack,
        aCompare,
        aBias || exports.GREATEST_LOWER_BOUND
      );
      if (index < 0) {
        return -1;
      }
      while (index - 1 >= 0) {
        if (aCompare(aHaystack[index], aHaystack[index - 1], true) !== 0) {
          break;
        }
        --index;
      }
      return index;
    };
  }
});

// source-map/lib/read-wasm.js
var require_read_wasm = __commonJS({
  "source-map/lib/read-wasm.js"(exports, module) {
    var mappingsWasm = null;
    module.exports = function readWasm() {
      if (typeof mappingsWasm === "string") {
        return fetch(mappingsWasm).then((response) => response.arrayBuffer());
      }
      if (mappingsWasm instanceof ArrayBuffer) {
        return Promise.resolve(mappingsWasm);
      }
      throw new Error("You must provide the string URL or ArrayBuffer contents of lib/mappings.wasm by calling SourceMapConsumer.initialize({ 'lib/mappings.wasm': ... }) before using SourceMapConsumer");
    };
    module.exports.initialize = (input) => mappingsWasm = input;
  }
});

// source-map/lib/wasm.js
var require_wasm = __commonJS({
  "source-map/lib/wasm.js"(exports, module) {
    var readWasm = require_read_wasm();
    function Mapping() {
      this.generatedLine = 0;
      this.generatedColumn = 0;
      this.lastGeneratedColumn = null;
      this.source = null;
      this.originalLine = null;
      this.originalColumn = null;
      this.name = null;
    }
    var cachedWasm = null;
    module.exports = function wasm() {
      if (cachedWasm) {
        return cachedWasm;
      }
      const callbackStack = [];
      cachedWasm = readWasm().then((buffer) => {
        return WebAssembly.instantiate(buffer, {
          env: {
            mapping_callback(generatedLine, generatedColumn, hasLastGeneratedColumn, lastGeneratedColumn, hasOriginal, source, originalLine, originalColumn, hasName, name) {
              const mapping = new Mapping();
              mapping.generatedLine = generatedLine + 1;
              mapping.generatedColumn = generatedColumn;
              if (hasLastGeneratedColumn) {
                mapping.lastGeneratedColumn = lastGeneratedColumn - 1;
              }
              if (hasOriginal) {
                mapping.source = source;
                mapping.originalLine = originalLine + 1;
                mapping.originalColumn = originalColumn;
                if (hasName) {
                  mapping.name = name;
                }
              }
              callbackStack[callbackStack.length - 1](mapping);
            },
            start_all_generated_locations_for() {
              console.time("all_generated_locations_for");
            },
            end_all_generated_locations_for() {
              console.timeEnd("all_generated_locations_for");
            },
            start_compute_column_spans() {
              console.time("compute_column_spans");
            },
            end_compute_column_spans() {
              console.timeEnd("compute_column_spans");
            },
            start_generated_location_for() {
              console.time("generated_location_for");
            },
            end_generated_location_for() {
              console.timeEnd("generated_location_for");
            },
            start_original_location_for() {
              console.time("original_location_for");
            },
            end_original_location_for() {
              console.timeEnd("original_location_for");
            },
            start_parse_mappings() {
              console.time("parse_mappings");
            },
            end_parse_mappings() {
              console.timeEnd("parse_mappings");
            },
            start_sort_by_generated_location() {
              console.time("sort_by_generated_location");
            },
            end_sort_by_generated_location() {
              console.timeEnd("sort_by_generated_location");
            },
            start_sort_by_original_location() {
              console.time("sort_by_original_location");
            },
            end_sort_by_original_location() {
              console.timeEnd("sort_by_original_location");
            }
          }
        });
      }).then((Wasm) => {
        return {
          exports: Wasm.instance.exports,
          withMappingCallback: (mappingCallback, f) => {
            callbackStack.push(mappingCallback);
            try {
              f();
            } finally {
              callbackStack.pop();
            }
          }
        };
      }).then(null, (e) => {
        cachedWasm = null;
        throw e;
      });
      return cachedWasm;
    };
  }
});

// source-map/lib/source-map-consumer.js
var require_source_map_consumer = __commonJS({
  "source-map/lib/source-map-consumer.js"(exports) {
    var util = require_util();
    var binarySearch = require_binary_search();
    var ArraySet = require_array_set().ArraySet;
    var base64VLQ = require_base64_vlq();
    var readWasm = require_read_wasm();
    var wasm = require_wasm();
    var INTERNAL = Symbol("smcInternal");
    var SourceMapConsumer2 = class {
      constructor(aSourceMap, aSourceMapURL) {
        if (aSourceMap == INTERNAL) {
          return Promise.resolve(this);
        }
        return _factory(aSourceMap, aSourceMapURL);
      }
      static initialize(opts) {
        readWasm.initialize(opts["lib/mappings.wasm"]);
      }
      static fromSourceMap(aSourceMap, aSourceMapURL) {
        return _factoryBSM(aSourceMap, aSourceMapURL);
      }
      static async with(rawSourceMap, sourceMapUrl, f) {
        const consumer = await new SourceMapConsumer2(rawSourceMap, sourceMapUrl);
        try {
          return await f(consumer);
        } finally {
          consumer.destroy();
        }
      }
      _parseMappings(aStr, aSourceRoot) {
        throw new Error("Subclasses must implement _parseMappings");
      }
      eachMapping(aCallback, aContext, aOrder) {
        throw new Error("Subclasses must implement eachMapping");
      }
      allGeneratedPositionsFor(aArgs) {
        throw new Error("Subclasses must implement allGeneratedPositionsFor");
      }
      destroy() {
        throw new Error("Subclasses must implement destroy");
      }
    };
    SourceMapConsumer2.prototype._version = 3;
    SourceMapConsumer2.GENERATED_ORDER = 1;
    SourceMapConsumer2.ORIGINAL_ORDER = 2;
    SourceMapConsumer2.GREATEST_LOWER_BOUND = 1;
    SourceMapConsumer2.LEAST_UPPER_BOUND = 2;
    exports.SourceMapConsumer = SourceMapConsumer2;
    var BasicSourceMapConsumer = class extends SourceMapConsumer2 {
      constructor(aSourceMap, aSourceMapURL) {
        return super(INTERNAL).then((that) => {
          let sourceMap = aSourceMap;
          if (typeof aSourceMap === "string") {
            sourceMap = util.parseSourceMapInput(aSourceMap);
          }
          const version = util.getArg(sourceMap, "version");
          let sources = util.getArg(sourceMap, "sources");
          const names = util.getArg(sourceMap, "names", []);
          let sourceRoot = util.getArg(sourceMap, "sourceRoot", null);
          const sourcesContent = util.getArg(sourceMap, "sourcesContent", null);
          const mappings = util.getArg(sourceMap, "mappings");
          const file = util.getArg(sourceMap, "file", null);
          if (version != that._version) {
            throw new Error("Unsupported version: " + version);
          }
          if (sourceRoot) {
            sourceRoot = util.normalize(sourceRoot);
          }
          sources = sources.map(String).map(util.normalize).map(function(source) {
            return sourceRoot && util.isAbsolute(sourceRoot) && util.isAbsolute(source) ? util.relative(sourceRoot, source) : source;
          });
          that._names = ArraySet.fromArray(names.map(String), true);
          that._sources = ArraySet.fromArray(sources, true);
          that._absoluteSources = that._sources.toArray().map(function(s) {
            return util.computeSourceURL(sourceRoot, s, aSourceMapURL);
          });
          that.sourceRoot = sourceRoot;
          that.sourcesContent = sourcesContent;
          that._mappings = mappings;
          that._sourceMapURL = aSourceMapURL;
          that.file = file;
          that._computedColumnSpans = false;
          that._mappingsPtr = 0;
          that._wasm = null;
          return wasm().then((w) => {
            that._wasm = w;
            return that;
          });
        });
      }
      _findSourceIndex(aSource) {
        let relativeSource = aSource;
        if (this.sourceRoot != null) {
          relativeSource = util.relative(this.sourceRoot, relativeSource);
        }
        if (this._sources.has(relativeSource)) {
          return this._sources.indexOf(relativeSource);
        }
        for (let i = 0; i < this._absoluteSources.length; ++i) {
          if (this._absoluteSources[i] == aSource) {
            return i;
          }
        }
        return -1;
      }
      static fromSourceMap(aSourceMap, aSourceMapURL) {
        return new BasicSourceMapConsumer(aSourceMap.toString());
      }
      get sources() {
        return this._absoluteSources.slice();
      }
      _getMappingsPtr() {
        if (this._mappingsPtr === 0) {
          this._parseMappings(this._mappings, this.sourceRoot);
        }
        return this._mappingsPtr;
      }
      _parseMappings(aStr, aSourceRoot) {
        const size = aStr.length;
        const mappingsBufPtr = this._wasm.exports.allocate_mappings(size);
        const mappingsBuf = new Uint8Array(this._wasm.exports.memory.buffer, mappingsBufPtr, size);
        for (let i = 0; i < size; i++) {
          mappingsBuf[i] = aStr.charCodeAt(i);
        }
        const mappingsPtr = this._wasm.exports.parse_mappings(mappingsBufPtr);
        if (!mappingsPtr) {
          const error = this._wasm.exports.get_last_error();
          let msg = `Error parsing mappings (code ${error}): `;
          switch (error) {
            case 1:
              msg += "the mappings contained a negative line, column, source index, or name index";
              break;
            case 2:
              msg += "the mappings contained a number larger than 2**32";
              break;
            case 3:
              msg += "reached EOF while in the middle of parsing a VLQ";
              break;
            case 4:
              msg += "invalid base 64 character while parsing a VLQ";
              break;
            default:
              msg += "unknown error code";
              break;
          }
          throw new Error(msg);
        }
        this._mappingsPtr = mappingsPtr;
      }
      eachMapping(aCallback, aContext, aOrder) {
        const context = aContext || null;
        const order = aOrder || SourceMapConsumer2.GENERATED_ORDER;
        const sourceRoot = this.sourceRoot;
        this._wasm.withMappingCallback(
          (mapping) => {
            if (mapping.source !== null) {
              mapping.source = this._sources.at(mapping.source);
              mapping.source = util.computeSourceURL(sourceRoot, mapping.source, this._sourceMapURL);
              if (mapping.name !== null) {
                mapping.name = this._names.at(mapping.name);
              }
            }
            aCallback.call(context, mapping);
          },
          () => {
            switch (order) {
              case SourceMapConsumer2.GENERATED_ORDER:
                this._wasm.exports.by_generated_location(this._getMappingsPtr());
                break;
              case SourceMapConsumer2.ORIGINAL_ORDER:
                this._wasm.exports.by_original_location(this._getMappingsPtr());
                break;
              default:
                throw new Error("Unknown order of iteration.");
            }
          }
        );
      }
      allGeneratedPositionsFor(aArgs) {
        let source = util.getArg(aArgs, "source");
        const originalLine = util.getArg(aArgs, "line");
        const originalColumn = aArgs.column || 0;
        source = this._findSourceIndex(source);
        if (source < 0) {
          return [];
        }
        if (originalLine < 1) {
          throw new Error("Line numbers must be >= 1");
        }
        if (originalColumn < 0) {
          throw new Error("Column numbers must be >= 0");
        }
        const mappings = [];
        this._wasm.withMappingCallback(
          (m) => {
            let lastColumn = m.lastGeneratedColumn;
            if (this._computedColumnSpans && lastColumn === null) {
              lastColumn = Infinity;
            }
            mappings.push({
              line: m.generatedLine,
              column: m.generatedColumn,
              lastColumn
            });
          },
          () => {
            this._wasm.exports.all_generated_locations_for(
              this._getMappingsPtr(),
              source,
              originalLine - 1,
              "column" in aArgs,
              originalColumn
            );
          }
        );
        return mappings;
      }
      destroy() {
        if (this._mappingsPtr !== 0) {
          this._wasm.exports.free_mappings(this._mappingsPtr);
          this._mappingsPtr = 0;
        }
      }
      computeColumnSpans() {
        if (this._computedColumnSpans) {
          return;
        }
        this._wasm.exports.compute_column_spans(this._getMappingsPtr());
        this._computedColumnSpans = true;
      }
      originalPositionFor(aArgs) {
        const needle = {
          generatedLine: util.getArg(aArgs, "line"),
          generatedColumn: util.getArg(aArgs, "column")
        };
        if (needle.generatedLine < 1) {
          throw new Error("Line numbers must be >= 1");
        }
        if (needle.generatedColumn < 0) {
          throw new Error("Column numbers must be >= 0");
        }
        let bias = util.getArg(aArgs, "bias", SourceMapConsumer2.GREATEST_LOWER_BOUND);
        if (bias == null) {
          bias = SourceMapConsumer2.GREATEST_LOWER_BOUND;
        }
        let mapping;
        this._wasm.withMappingCallback((m) => mapping = m, () => {
          this._wasm.exports.original_location_for(
            this._getMappingsPtr(),
            needle.generatedLine - 1,
            needle.generatedColumn,
            bias
          );
        });
        if (mapping) {
          if (mapping.generatedLine === needle.generatedLine) {
            let source = util.getArg(mapping, "source", null);
            if (source !== null) {
              source = this._sources.at(source);
              source = util.computeSourceURL(this.sourceRoot, source, this._sourceMapURL);
            }
            let name = util.getArg(mapping, "name", null);
            if (name !== null) {
              name = this._names.at(name);
            }
            return {
              source,
              line: util.getArg(mapping, "originalLine", null),
              column: util.getArg(mapping, "originalColumn", null),
              name
            };
          }
        }
        return {
          source: null,
          line: null,
          column: null,
          name: null
        };
      }
      hasContentsOfAllSources() {
        if (!this.sourcesContent) {
          return false;
        }
        return this.sourcesContent.length >= this._sources.size() && !this.sourcesContent.some(function(sc) {
          return sc == null;
        });
      }
      sourceContentFor(aSource, nullOnMissing) {
        if (!this.sourcesContent) {
          return null;
        }
        const index = this._findSourceIndex(aSource);
        if (index >= 0) {
          return this.sourcesContent[index];
        }
        let relativeSource = aSource;
        if (this.sourceRoot != null) {
          relativeSource = util.relative(this.sourceRoot, relativeSource);
        }
        let url;
        if (this.sourceRoot != null && (url = util.urlParse(this.sourceRoot))) {
          const fileUriAbsPath = relativeSource.replace(/^file:\/\//, "");
          if (url.scheme == "file" && this._sources.has(fileUriAbsPath)) {
            return this.sourcesContent[this._sources.indexOf(fileUriAbsPath)];
          }
          if ((!url.path || url.path == "/") && this._sources.has("/" + relativeSource)) {
            return this.sourcesContent[this._sources.indexOf("/" + relativeSource)];
          }
        }
        if (nullOnMissing) {
          return null;
        }
        throw new Error('"' + relativeSource + '" is not in the SourceMap.');
      }
      generatedPositionFor(aArgs) {
        let source = util.getArg(aArgs, "source");
        source = this._findSourceIndex(source);
        if (source < 0) {
          return {
            line: null,
            column: null,
            lastColumn: null
          };
        }
        const needle = {
          source,
          originalLine: util.getArg(aArgs, "line"),
          originalColumn: util.getArg(aArgs, "column")
        };
        if (needle.originalLine < 1) {
          throw new Error("Line numbers must be >= 1");
        }
        if (needle.originalColumn < 0) {
          throw new Error("Column numbers must be >= 0");
        }
        let bias = util.getArg(aArgs, "bias", SourceMapConsumer2.GREATEST_LOWER_BOUND);
        if (bias == null) {
          bias = SourceMapConsumer2.GREATEST_LOWER_BOUND;
        }
        let mapping;
        this._wasm.withMappingCallback((m) => mapping = m, () => {
          this._wasm.exports.generated_location_for(
            this._getMappingsPtr(),
            needle.source,
            needle.originalLine - 1,
            needle.originalColumn,
            bias
          );
        });
        if (mapping) {
          if (mapping.source === needle.source) {
            let lastColumn = mapping.lastGeneratedColumn;
            if (this._computedColumnSpans && lastColumn === null) {
              lastColumn = Infinity;
            }
            return {
              line: util.getArg(mapping, "generatedLine", null),
              column: util.getArg(mapping, "generatedColumn", null),
              lastColumn
            };
          }
        }
        return {
          line: null,
          column: null,
          lastColumn: null
        };
      }
    };
    BasicSourceMapConsumer.prototype.consumer = SourceMapConsumer2;
    exports.BasicSourceMapConsumer = BasicSourceMapConsumer;
    var IndexedSourceMapConsumer = class extends SourceMapConsumer2 {
      constructor(aSourceMap, aSourceMapURL) {
        return super(INTERNAL).then((that) => {
          let sourceMap = aSourceMap;
          if (typeof aSourceMap === "string") {
            sourceMap = util.parseSourceMapInput(aSourceMap);
          }
          const version = util.getArg(sourceMap, "version");
          const sections = util.getArg(sourceMap, "sections");
          if (version != that._version) {
            throw new Error("Unsupported version: " + version);
          }
          that._sources = new ArraySet();
          that._names = new ArraySet();
          that.__generatedMappings = null;
          that.__originalMappings = null;
          that.__generatedMappingsUnsorted = null;
          that.__originalMappingsUnsorted = null;
          let lastOffset = {
            line: -1,
            column: 0
          };
          return Promise.all(sections.map((s) => {
            if (s.url) {
              throw new Error("Support for url field in sections not implemented.");
            }
            const offset = util.getArg(s, "offset");
            const offsetLine = util.getArg(offset, "line");
            const offsetColumn = util.getArg(offset, "column");
            if (offsetLine < lastOffset.line || offsetLine === lastOffset.line && offsetColumn < lastOffset.column) {
              throw new Error("Section offsets must be ordered and non-overlapping.");
            }
            lastOffset = offset;
            const cons = new SourceMapConsumer2(util.getArg(s, "map"), aSourceMapURL);
            return cons.then((consumer) => {
              return {
                generatedOffset: {
                  generatedLine: offsetLine + 1,
                  generatedColumn: offsetColumn + 1
                },
                consumer
              };
            });
          })).then((s) => {
            that._sections = s;
            return that;
          });
        });
      }
      get _generatedMappings() {
        if (!this.__generatedMappings) {
          this._sortGeneratedMappings();
        }
        return this.__generatedMappings;
      }
      get _originalMappings() {
        if (!this.__originalMappings) {
          this._sortOriginalMappings();
        }
        return this.__originalMappings;
      }
      get _generatedMappingsUnsorted() {
        if (!this.__generatedMappingsUnsorted) {
          this._parseMappings(this._mappings, this.sourceRoot);
        }
        return this.__generatedMappingsUnsorted;
      }
      get _originalMappingsUnsorted() {
        if (!this.__originalMappingsUnsorted) {
          this._parseMappings(this._mappings, this.sourceRoot);
        }
        return this.__originalMappingsUnsorted;
      }
      _sortGeneratedMappings() {
        const mappings = this._generatedMappingsUnsorted;
        mappings.sort(util.compareByGeneratedPositionsDeflated);
        this.__generatedMappings = mappings;
      }
      _sortOriginalMappings() {
        const mappings = this._originalMappingsUnsorted;
        mappings.sort(util.compareByOriginalPositions);
        this.__originalMappings = mappings;
      }
      get sources() {
        const sources = [];
        for (let i = 0; i < this._sections.length; i++) {
          for (let j = 0; j < this._sections[i].consumer.sources.length; j++) {
            sources.push(this._sections[i].consumer.sources[j]);
          }
        }
        return sources;
      }
      originalPositionFor(aArgs) {
        const needle = {
          generatedLine: util.getArg(aArgs, "line"),
          generatedColumn: util.getArg(aArgs, "column")
        };
        const sectionIndex = binarySearch.search(
          needle,
          this._sections,
          function(aNeedle, section2) {
            const cmp = aNeedle.generatedLine - section2.generatedOffset.generatedLine;
            if (cmp) {
              return cmp;
            }
            return aNeedle.generatedColumn - section2.generatedOffset.generatedColumn;
          }
        );
        const section = this._sections[sectionIndex];
        if (!section) {
          return {
            source: null,
            line: null,
            column: null,
            name: null
          };
        }
        return section.consumer.originalPositionFor({
          line: needle.generatedLine - (section.generatedOffset.generatedLine - 1),
          column: needle.generatedColumn - (section.generatedOffset.generatedLine === needle.generatedLine ? section.generatedOffset.generatedColumn - 1 : 0),
          bias: aArgs.bias
        });
      }
      hasContentsOfAllSources() {
        return this._sections.every(function(s) {
          return s.consumer.hasContentsOfAllSources();
        });
      }
      sourceContentFor(aSource, nullOnMissing) {
        for (let i = 0; i < this._sections.length; i++) {
          const section = this._sections[i];
          const content = section.consumer.sourceContentFor(aSource, true);
          if (content) {
            return content;
          }
        }
        if (nullOnMissing) {
          return null;
        }
        throw new Error('"' + aSource + '" is not in the SourceMap.');
      }
      generatedPositionFor(aArgs) {
        for (let i = 0; i < this._sections.length; i++) {
          const section = this._sections[i];
          if (section.consumer._findSourceIndex(util.getArg(aArgs, "source")) === -1) {
            continue;
          }
          const generatedPosition = section.consumer.generatedPositionFor(aArgs);
          if (generatedPosition) {
            const ret = {
              line: generatedPosition.line + (section.generatedOffset.generatedLine - 1),
              column: generatedPosition.column + (section.generatedOffset.generatedLine === generatedPosition.line ? section.generatedOffset.generatedColumn - 1 : 0)
            };
            return ret;
          }
        }
        return {
          line: null,
          column: null
        };
      }
      _parseMappings(aStr, aSourceRoot) {
        const generatedMappings = this.__generatedMappingsUnsorted = [];
        const originalMappings = this.__originalMappingsUnsorted = [];
        for (let i = 0; i < this._sections.length; i++) {
          const section = this._sections[i];
          const sectionMappings = [];
          section.consumer.eachMapping((m) => sectionMappings.push(m));
          for (let j = 0; j < sectionMappings.length; j++) {
            const mapping = sectionMappings[j];
            let source = util.computeSourceURL(section.consumer.sourceRoot, null, this._sourceMapURL);
            this._sources.add(source);
            source = this._sources.indexOf(source);
            let name = null;
            if (mapping.name) {
              this._names.add(mapping.name);
              name = this._names.indexOf(mapping.name);
            }
            const adjustedMapping = {
              source,
              generatedLine: mapping.generatedLine + (section.generatedOffset.generatedLine - 1),
              generatedColumn: mapping.generatedColumn + (section.generatedOffset.generatedLine === mapping.generatedLine ? section.generatedOffset.generatedColumn - 1 : 0),
              originalLine: mapping.originalLine,
              originalColumn: mapping.originalColumn,
              name
            };
            generatedMappings.push(adjustedMapping);
            if (typeof adjustedMapping.originalLine === "number") {
              originalMappings.push(adjustedMapping);
            }
          }
        }
      }
      eachMapping(aCallback, aContext, aOrder) {
        const context = aContext || null;
        const order = aOrder || SourceMapConsumer2.GENERATED_ORDER;
        let mappings;
        switch (order) {
          case SourceMapConsumer2.GENERATED_ORDER:
            mappings = this._generatedMappings;
            break;
          case SourceMapConsumer2.ORIGINAL_ORDER:
            mappings = this._originalMappings;
            break;
          default:
            throw new Error("Unknown order of iteration.");
        }
        const sourceRoot = this.sourceRoot;
        mappings.map(function(mapping) {
          let source = null;
          if (mapping.source !== null) {
            source = this._sources.at(mapping.source);
            source = util.computeSourceURL(sourceRoot, source, this._sourceMapURL);
          }
          return {
            source,
            generatedLine: mapping.generatedLine,
            generatedColumn: mapping.generatedColumn,
            originalLine: mapping.originalLine,
            originalColumn: mapping.originalColumn,
            name: mapping.name === null ? null : this._names.at(mapping.name)
          };
        }, this).forEach(aCallback, context);
      }
      _findMapping(aNeedle, aMappings, aLineName, aColumnName, aComparator, aBias) {
        if (aNeedle[aLineName] <= 0) {
          throw new TypeError("Line must be greater than or equal to 1, got " + aNeedle[aLineName]);
        }
        if (aNeedle[aColumnName] < 0) {
          throw new TypeError("Column must be greater than or equal to 0, got " + aNeedle[aColumnName]);
        }
        return binarySearch.search(aNeedle, aMappings, aComparator, aBias);
      }
      allGeneratedPositionsFor(aArgs) {
        const line = util.getArg(aArgs, "line");
        const needle = {
          source: util.getArg(aArgs, "source"),
          originalLine: line,
          originalColumn: util.getArg(aArgs, "column", 0)
        };
        needle.source = this._findSourceIndex(needle.source);
        if (needle.source < 0) {
          return [];
        }
        if (needle.originalLine < 1) {
          throw new Error("Line numbers must be >= 1");
        }
        if (needle.originalColumn < 0) {
          throw new Error("Column numbers must be >= 0");
        }
        const mappings = [];
        let index = this._findMapping(
          needle,
          this._originalMappings,
          "originalLine",
          "originalColumn",
          util.compareByOriginalPositions,
          binarySearch.LEAST_UPPER_BOUND
        );
        if (index >= 0) {
          let mapping = this._originalMappings[index];
          if (aArgs.column === void 0) {
            const originalLine = mapping.originalLine;
            while (mapping && mapping.originalLine === originalLine) {
              let lastColumn = mapping.lastGeneratedColumn;
              if (this._computedColumnSpans && lastColumn === null) {
                lastColumn = Infinity;
              }
              mappings.push({
                line: util.getArg(mapping, "generatedLine", null),
                column: util.getArg(mapping, "generatedColumn", null),
                lastColumn
              });
              mapping = this._originalMappings[++index];
            }
          } else {
            const originalColumn = mapping.originalColumn;
            while (mapping && mapping.originalLine === line && mapping.originalColumn == originalColumn) {
              let lastColumn = mapping.lastGeneratedColumn;
              if (this._computedColumnSpans && lastColumn === null) {
                lastColumn = Infinity;
              }
              mappings.push({
                line: util.getArg(mapping, "generatedLine", null),
                column: util.getArg(mapping, "generatedColumn", null),
                lastColumn
              });
              mapping = this._originalMappings[++index];
            }
          }
        }
        return mappings;
      }
      destroy() {
        for (let i = 0; i < this._sections.length; i++) {
          this._sections[i].consumer.destroy();
        }
      }
    };
    exports.IndexedSourceMapConsumer = IndexedSourceMapConsumer;
    function _factory(aSourceMap, aSourceMapURL) {
      let sourceMap = aSourceMap;
      if (typeof aSourceMap === "string") {
        sourceMap = util.parseSourceMapInput(aSourceMap);
      }
      const consumer = sourceMap.sections != null ? new IndexedSourceMapConsumer(sourceMap, aSourceMapURL) : new BasicSourceMapConsumer(sourceMap, aSourceMapURL);
      return Promise.resolve(consumer);
    }
    function _factoryBSM(aSourceMap, aSourceMapURL) {
      return BasicSourceMapConsumer.fromSourceMap(aSourceMap, aSourceMapURL);
    }
  }
});

// source-map/lib/source-node.js
var require_source_node = __commonJS({
  "source-map/lib/source-node.js"(exports) {
    var SourceMapGenerator = require_source_map_generator().SourceMapGenerator;
    var util = require_util();
    var REGEX_NEWLINE = /(\r?\n)/;
    var NEWLINE_CODE = 10;
    var isSourceNode = "$$$isSourceNode$$$";
    var SourceNode = class {
      constructor(aLine, aColumn, aSource, aChunks, aName) {
        this.children = [];
        this.sourceContents = {};
        this.line = aLine == null ? null : aLine;
        this.column = aColumn == null ? null : aColumn;
        this.source = aSource == null ? null : aSource;
        this.name = aName == null ? null : aName;
        this[isSourceNode] = true;
        if (aChunks != null)
          this.add(aChunks);
      }
      static fromStringWithSourceMap(aGeneratedCode, aSourceMapConsumer, aRelativePath) {
        const node = new SourceNode();
        const remainingLines = aGeneratedCode.split(REGEX_NEWLINE);
        let remainingLinesIndex = 0;
        const shiftNextLine = function() {
          const lineContents = getNextLine();
          const newLine = getNextLine() || "";
          return lineContents + newLine;
          function getNextLine() {
            return remainingLinesIndex < remainingLines.length ? remainingLines[remainingLinesIndex++] : void 0;
          }
        };
        let lastGeneratedLine = 1, lastGeneratedColumn = 0;
        let lastMapping = null;
        let nextLine;
        aSourceMapConsumer.eachMapping(function(mapping) {
          if (lastMapping !== null) {
            if (lastGeneratedLine < mapping.generatedLine) {
              addMappingWithCode(lastMapping, shiftNextLine());
              lastGeneratedLine++;
              lastGeneratedColumn = 0;
            } else {
              nextLine = remainingLines[remainingLinesIndex] || "";
              const code = nextLine.substr(0, mapping.generatedColumn - lastGeneratedColumn);
              remainingLines[remainingLinesIndex] = nextLine.substr(mapping.generatedColumn - lastGeneratedColumn);
              lastGeneratedColumn = mapping.generatedColumn;
              addMappingWithCode(lastMapping, code);
              lastMapping = mapping;
              return;
            }
          }
          while (lastGeneratedLine < mapping.generatedLine) {
            node.add(shiftNextLine());
            lastGeneratedLine++;
          }
          if (lastGeneratedColumn < mapping.generatedColumn) {
            nextLine = remainingLines[remainingLinesIndex] || "";
            node.add(nextLine.substr(0, mapping.generatedColumn));
            remainingLines[remainingLinesIndex] = nextLine.substr(mapping.generatedColumn);
            lastGeneratedColumn = mapping.generatedColumn;
          }
          lastMapping = mapping;
        }, this);
        if (remainingLinesIndex < remainingLines.length) {
          if (lastMapping) {
            addMappingWithCode(lastMapping, shiftNextLine());
          }
          node.add(remainingLines.splice(remainingLinesIndex).join(""));
        }
        aSourceMapConsumer.sources.forEach(function(sourceFile) {
          const content = aSourceMapConsumer.sourceContentFor(sourceFile);
          if (content != null) {
            if (aRelativePath != null) {
              sourceFile = util.join(aRelativePath, sourceFile);
            }
            node.setSourceContent(sourceFile, content);
          }
        });
        return node;
        function addMappingWithCode(mapping, code) {
          if (mapping === null || mapping.source === void 0) {
            node.add(code);
          } else {
            const source = aRelativePath ? util.join(aRelativePath, mapping.source) : mapping.source;
            node.add(new SourceNode(
              mapping.originalLine,
              mapping.originalColumn,
              source,
              code,
              mapping.name
            ));
          }
        }
      }
      add(aChunk) {
        if (Array.isArray(aChunk)) {
          aChunk.forEach(function(chunk) {
            this.add(chunk);
          }, this);
        } else if (aChunk[isSourceNode] || typeof aChunk === "string") {
          if (aChunk) {
            this.children.push(aChunk);
          }
        } else {
          throw new TypeError(
            "Expected a SourceNode, string, or an array of SourceNodes and strings. Got " + aChunk
          );
        }
        return this;
      }
      prepend(aChunk) {
        if (Array.isArray(aChunk)) {
          for (let i = aChunk.length - 1; i >= 0; i--) {
            this.prepend(aChunk[i]);
          }
        } else if (aChunk[isSourceNode] || typeof aChunk === "string") {
          this.children.unshift(aChunk);
        } else {
          throw new TypeError(
            "Expected a SourceNode, string, or an array of SourceNodes and strings. Got " + aChunk
          );
        }
        return this;
      }
      walk(aFn) {
        let chunk;
        for (let i = 0, len = this.children.length; i < len; i++) {
          chunk = this.children[i];
          if (chunk[isSourceNode]) {
            chunk.walk(aFn);
          } else if (chunk !== "") {
            aFn(chunk, {
              source: this.source,
              line: this.line,
              column: this.column,
              name: this.name
            });
          }
        }
      }
      join(aSep) {
        let newChildren;
        let i;
        const len = this.children.length;
        if (len > 0) {
          newChildren = [];
          for (i = 0; i < len - 1; i++) {
            newChildren.push(this.children[i]);
            newChildren.push(aSep);
          }
          newChildren.push(this.children[i]);
          this.children = newChildren;
        }
        return this;
      }
      replaceRight(aPattern, aReplacement) {
        const lastChild = this.children[this.children.length - 1];
        if (lastChild[isSourceNode]) {
          lastChild.replaceRight(aPattern, aReplacement);
        } else if (typeof lastChild === "string") {
          this.children[this.children.length - 1] = lastChild.replace(aPattern, aReplacement);
        } else {
          this.children.push("".replace(aPattern, aReplacement));
        }
        return this;
      }
      setSourceContent(aSourceFile, aSourceContent) {
        this.sourceContents[util.toSetString(aSourceFile)] = aSourceContent;
      }
      walkSourceContents(aFn) {
        for (let i = 0, len = this.children.length; i < len; i++) {
          if (this.children[i][isSourceNode]) {
            this.children[i].walkSourceContents(aFn);
          }
        }
        const sources = Object.keys(this.sourceContents);
        for (let i = 0, len = sources.length; i < len; i++) {
          aFn(util.fromSetString(sources[i]), this.sourceContents[sources[i]]);
        }
      }
      toString() {
        let str = "";
        this.walk(function(chunk) {
          str += chunk;
        });
        return str;
      }
      toStringWithSourceMap(aArgs) {
        const generated = {
          code: "",
          line: 1,
          column: 0
        };
        const map = new SourceMapGenerator(aArgs);
        let sourceMappingActive = false;
        let lastOriginalSource = null;
        let lastOriginalLine = null;
        let lastOriginalColumn = null;
        let lastOriginalName = null;
        this.walk(function(chunk, original) {
          generated.code += chunk;
          if (original.source !== null && original.line !== null && original.column !== null) {
            if (lastOriginalSource !== original.source || lastOriginalLine !== original.line || lastOriginalColumn !== original.column || lastOriginalName !== original.name) {
              map.addMapping({
                source: original.source,
                original: {
                  line: original.line,
                  column: original.column
                },
                generated: {
                  line: generated.line,
                  column: generated.column
                },
                name: original.name
              });
            }
            lastOriginalSource = original.source;
            lastOriginalLine = original.line;
            lastOriginalColumn = original.column;
            lastOriginalName = original.name;
            sourceMappingActive = true;
          } else if (sourceMappingActive) {
            map.addMapping({
              generated: {
                line: generated.line,
                column: generated.column
              }
            });
            lastOriginalSource = null;
            sourceMappingActive = false;
          }
          for (let idx = 0, length = chunk.length; idx < length; idx++) {
            if (chunk.charCodeAt(idx) === NEWLINE_CODE) {
              generated.line++;
              generated.column = 0;
              if (idx + 1 === length) {
                lastOriginalSource = null;
                sourceMappingActive = false;
              } else if (sourceMappingActive) {
                map.addMapping({
                  source: original.source,
                  original: {
                    line: original.line,
                    column: original.column
                  },
                  generated: {
                    line: generated.line,
                    column: generated.column
                  },
                  name: original.name
                });
              }
            } else {
              generated.column++;
            }
          }
        });
        this.walkSourceContents(function(sourceFile, sourceContent) {
          map.setSourceContent(sourceFile, sourceContent);
        });
        return { code: generated.code, map };
      }
    };
    exports.SourceNode = SourceNode;
  }
});

// source-map/source-map.js
var require_source_map = __commonJS({
  "source-map/source-map.js"(exports) {
    exports.SourceMapGenerator = require_source_map_generator().SourceMapGenerator;
    exports.SourceMapConsumer = require_source_map_consumer().SourceMapConsumer;
    exports.SourceNode = require_source_node().SourceNode;
  }
});

// js/main.js
var import_alt_client9 = __toESM(require_alt_client(), 1);

// altv-esbuild-rust-wasm:../rust_wasm/pkg/rust_wasm_bg.wasm
var import_alt_client = __toESM(require_alt_client());
var wasmLoader = (altv_imports) => {
  let wasm_bindgen;
  const __exports = {};
  let script_src;
  if (typeof document !== "undefined" && document.currentScript !== null) {
    script_src = new URL(document.currentScript.src, location.href).toString();
  }
  let wasm = void 0;
  const cachedTextDecoder = typeof TextDecoder !== "undefined" ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true }) : { decode: () => {
    throw Error("TextDecoder not available");
  } };
  if (typeof TextDecoder !== "undefined") {
    cachedTextDecoder.decode();
  }
  ;
  let cachedUint8ArrayMemory0 = null;
  function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
      cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
  }
  function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
  }
  const heap = new Array(128).fill(void 0);
  heap.push(void 0, null, true, false);
  let heap_next = heap.length;
  function addHeapObject(obj) {
    if (heap_next === heap.length)
      heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];
    if (typeof heap_next !== "number")
      throw new Error("corrupt heap");
    heap[idx] = obj;
    return idx;
  }
  function getObject(idx) {
    return heap[idx];
  }
  function _assertBoolean(n) {
    if (typeof n !== "boolean") {
      throw new Error(`expected a boolean argument, found ${typeof n}`);
    }
  }
  function isLikeNone(x) {
    return x === void 0 || x === null;
  }
  function _assertNum(n) {
    if (typeof n !== "number")
      throw new Error(`expected a number argument, found ${typeof n}`);
  }
  let cachedDataViewMemory0 = null;
  function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || cachedDataViewMemory0.buffer.detached === void 0 && cachedDataViewMemory0.buffer !== wasm.memory.buffer) {
      cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
  }
  let WASM_VECTOR_LEN = 0;
  const cachedTextEncoder = typeof TextEncoder !== "undefined" ? new TextEncoder("utf-8") : { encode: () => {
    throw Error("TextEncoder not available");
  } };
  const encodeString = typeof cachedTextEncoder.encodeInto === "function" ? function(arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
  } : function(arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
      read: arg.length,
      written: buf.length
    };
  };
  function passStringToWasm0(arg, malloc, realloc) {
    if (typeof arg !== "string")
      throw new Error(`expected a string argument, found ${typeof arg}`);
    if (realloc === void 0) {
      const buf = cachedTextEncoder.encode(arg);
      const ptr2 = malloc(buf.length, 1) >>> 0;
      getUint8ArrayMemory0().subarray(ptr2, ptr2 + buf.length).set(buf);
      WASM_VECTOR_LEN = buf.length;
      return ptr2;
    }
    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;
    const mem = getUint8ArrayMemory0();
    let offset = 0;
    for (; offset < len; offset++) {
      const code = arg.charCodeAt(offset);
      if (code > 127)
        break;
      mem[ptr + offset] = code;
    }
    if (offset !== len) {
      if (offset !== 0) {
        arg = arg.slice(offset);
      }
      ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
      const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
      const ret = encodeString(arg, view);
      if (ret.read !== arg.length)
        throw new Error("failed to pass whole string");
      offset += ret.written;
      ptr = realloc(ptr, len, offset, 1) >>> 0;
    }
    WASM_VECTOR_LEN = offset;
    return ptr;
  }
  function debugString(val) {
    const type = typeof val;
    if (type == "number" || type == "boolean" || val == null) {
      return `${val}`;
    }
    if (type == "string") {
      return `"${val}"`;
    }
    if (type == "symbol") {
      const description = val.description;
      if (description == null) {
        return "Symbol";
      } else {
        return `Symbol(${description})`;
      }
    }
    if (type == "function") {
      const name = val.name;
      if (typeof name == "string" && name.length > 0) {
        return `Function(${name})`;
      } else {
        return "Function";
      }
    }
    if (Array.isArray(val)) {
      const length = val.length;
      let debug = "[";
      if (length > 0) {
        debug += debugString(val[0]);
      }
      for (let i = 1; i < length; i++) {
        debug += ", " + debugString(val[i]);
      }
      debug += "]";
      return debug;
    }
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
      className = builtInMatches[1];
    } else {
      return toString.call(val);
    }
    if (className == "Object") {
      try {
        return "Object(" + JSON.stringify(val) + ")";
      } catch (_) {
        return "Object";
      }
    }
    if (val instanceof Error) {
      return `${val.name}: ${val.message}
${val.stack}`;
    }
    return className;
  }
  function _assertBigInt(n) {
    if (typeof n !== "bigint")
      throw new Error(`expected a bigint argument, found ${typeof n}`);
  }
  function dropObject(idx) {
    if (idx < 132)
      return;
    heap[idx] = heap_next;
    heap_next = idx;
  }
  function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
  }
  function logError(f, args) {
    try {
      return f.apply(this, args);
    } catch (e) {
      let error = function() {
        try {
          return e instanceof Error ? `${e.message}

Stack:
${e.stack}` : e.toString();
        } catch (_) {
          return "<failed to stringify thrown value>";
        }
      }();
      console.error("wasm-bindgen: imported JS function that was not marked as `catch` threw an error:", error);
      throw e;
    }
  }
  function notDefined(what) {
    return () => {
      throw new Error(`${what} is not defined`);
    };
  }
  __exports.on_altv_event = function(event) {
    wasm.on_altv_event(addHeapObject(event));
  };
  __exports.test_altv_events = function() {
    wasm.test_altv_events();
  };
  __exports.test_altv_events2 = function() {
    wasm.test_altv_events2();
  };
  __exports.test_local_player = function() {
    wasm.test_local_player();
  };
  __exports.get_current_panic_info = function() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.get_current_panic_info(retptr);
      var r0 = getDataViewMemory0().getInt32(retptr + 4 * 0, true);
      var r1 = getDataViewMemory0().getInt32(retptr + 4 * 1, true);
      let v1;
      if (r0 !== 0) {
        v1 = getStringFromWasm0(r0, r1).slice();
        wasm.__wbindgen_free(r0, r1 * 1, 1);
      }
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  };
  __exports.main = function() {
    wasm.main();
  };
  __exports.on_every_tick = function() {
    wasm.on_every_tick();
  };
  __exports.test_vehicle = function() {
    wasm.test_vehicle();
  };
  __exports.on_script_event = function(event) {
    wasm.on_script_event(addHeapObject(event));
  };
  __exports.test_script_events = function() {
    wasm.test_script_events();
  };
  __exports.test_timers = function() {
    wasm.test_timers();
  };
  function handleError(f, args) {
    try {
      return f.apply(this, args);
    } catch (e) {
      wasm.__wbindgen_exn_store(addHeapObject(e));
    }
  }
  async function __wbg_load(module, imports) {
    if (typeof Response === "function" && module instanceof Response) {
      if (typeof WebAssembly.instantiateStreaming === "function") {
        try {
          return await WebAssembly.instantiateStreaming(module, imports);
        } catch (e) {
          if (module.headers.get("Content-Type") != "application/wasm") {
            console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
          } else {
            throw e;
          }
        }
      }
      const bytes = await module.arrayBuffer();
      return await WebAssembly.instantiate(bytes, imports);
    } else {
      const instance = await WebAssembly.instantiate(module, imports);
      if (instance instanceof WebAssembly.Instance) {
        return { instance, module };
      } else {
        return instance;
      }
    }
  }
  function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg_loginfo_98f61df18ff41631 = function() {
      return logError(function(arg0, arg1) {
        altv_imports.log_info(getStringFromWasm0(arg0, arg1));
      }, arguments);
    };
    imports.wbg.__wbg_logwarn_74842ee54ecb2cdb = function() {
      return logError(function(arg0, arg1) {
        altv_imports.log_warn(getStringFromWasm0(arg0, arg1));
      }, arguments);
    };
    imports.wbg.__wbg_logerror_9fb4485ac2fceaf3 = function() {
      return logError(function(arg0, arg1) {
        altv_imports.log_error(getStringFromWasm0(arg0, arg1));
      }, arguments);
    };
    imports.wbg.__wbg_enablealtvevent_6e9e70c6a72d06b0 = function() {
      return logError(function(arg0, arg1) {
        altv_imports.enable_altv_event(getStringFromWasm0(arg0, arg1));
      }, arguments);
    };
    imports.wbg.__wbg_disablealtvevent_59783cfbfa7879f5 = function() {
      return logError(function(arg0, arg1) {
        altv_imports.disable_altv_event(getStringFromWasm0(arg0, arg1));
      }, arguments);
    };
    imports.wbg.__wbg_getbaseobjectref_9e330ecf63860973 = function() {
      return logError(function(arg0, arg1, arg2) {
        const ret = altv_imports.get_base_object_ref(arg0, arg1 !== 0, arg2 >>> 0);
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_id_a86d821a0829fbc3 = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).id;
        _assertNum(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_pos_60ea425417fc44ca = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).pos;
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_setpos_690248f688d9ed5a = function() {
      return logError(function(arg0, arg1) {
        getObject(arg0).pos = takeObject(arg1);
      }, arguments);
    };
    imports.wbg.__wbg_dimension_072cd04186e4725b = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).dimension;
        _assertNum(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_netOwner_8a1dfdc5fcf3c8b6 = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).netOwner;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_name_1716b4c36dc71476 = function() {
      return logError(function(arg0, arg1) {
        const ret = getObject(arg1).name;
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
      }, arguments);
    };
    imports.wbg.__wbg_emitlocaleventrust_8b23b0f27d378dc5 = function() {
      return logError(function(arg0, arg1, arg2) {
        altv_imports.emit_local_event_rust(getStringFromWasm0(arg0, arg1), takeObject(arg2));
      }, arguments);
    };
    imports.wbg.__wbg_emitlocaleventjs_c804c570d9d49d25 = function() {
      return logError(function(arg0, arg1, arg2) {
        altv_imports.emit_local_event_js(getStringFromWasm0(arg0, arg1), takeObject(arg2));
      }, arguments);
    };
    imports.wbg.__wbg_getstreamedinvehicles_237c3281c4d9a912 = function() {
      return logError(function() {
        const ret = altv_imports.get_streamed_in_vehicles();
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_getnettime_eb7127b6c035df7e = typeof altv_imports.get_net_time == "function" ? altv_imports.get_net_time : notDefined("altv_imports.get_net_time");
    imports.wbg.__wbg_getbaseobjectrawhandle_6d13adbe8ef1beb6 = function() {
      return logError(function(arg0) {
        const ret = altv_imports.get_base_object_raw_handle(getObject(arg0));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_getlocalplayer_17b45423263ae9c6 = function() {
      return logError(function() {
        const ret = altv_imports.get_local_player();
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_islocalplayer_69ddc7e4b2d0ea56 = function() {
      return logError(function(arg0) {
        const ret = altv_imports.is_local_player(getObject(arg0));
        _assertBoolean(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
      const ret = new Error(getStringFromWasm0(arg0, arg1));
      return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_is_undefined = function(arg0) {
      const ret = getObject(arg0) === void 0;
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbindgen_as_number = function(arg0) {
      const ret = +getObject(arg0);
      return ret;
    };
    imports.wbg.__wbindgen_in = function(arg0, arg1) {
      const ret = getObject(arg0) in getObject(arg1);
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "number" ? obj : void 0;
      if (!isLikeNone(ret)) {
        _assertNum(ret);
      }
      getDataViewMemory0().setFloat64(arg0 + 8 * 1, isLikeNone(ret) ? 0 : ret, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
      const v = getObject(arg0);
      const ret = typeof v === "boolean" ? v ? 1 : 0 : 2;
      _assertNum(ret);
      return ret;
    };
    imports.wbg.__wbindgen_number_new = function(arg0) {
      const ret = arg0;
      return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_get = function(arg0, arg1) {
      const obj = getObject(arg1);
      const ret = typeof obj === "string" ? obj : void 0;
      var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      var len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_is_bigint = function(arg0) {
      const ret = typeof getObject(arg0) === "bigint";
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbindgen_is_object = function(arg0) {
      const val = getObject(arg0);
      const ret = typeof val === "object" && val !== null;
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbindgen_is_string = function(arg0) {
      const ret = typeof getObject(arg0) === "string";
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
      const ret = getObject(arg0);
      return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_eq = function(arg0, arg1) {
      const ret = getObject(arg0) === getObject(arg1);
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbindgen_bigint_from_u64 = function(arg0) {
      const ret = BigInt.asUintN(64, arg0);
      return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
      const ret = getStringFromWasm0(arg0, arg1);
      return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_jsval_loose_eq = function(arg0, arg1) {
      const ret = getObject(arg0) == getObject(arg1);
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbindgen_bigint_from_i64 = function(arg0) {
      const ret = arg0;
      return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_bigint_from_i128 = function(arg0, arg1) {
      const ret = arg0 << BigInt(64) | BigInt.asUintN(64, arg1);
      return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_bigint_from_u128 = function(arg0, arg1) {
      const ret = BigInt.asUintN(64, arg0) << BigInt(64) | BigInt.asUintN(64, arg1);
      return addHeapObject(ret);
    };
    imports.wbg.__wbg_String_b9412f8799faab3e = function() {
      return logError(function(arg0, arg1) {
        const ret = String(getObject(arg1));
        const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
      }, arguments);
    };
    imports.wbg.__wbg_getwithrefkey_edc2c8960f0f1191 = function() {
      return logError(function(arg0, arg1) {
        const ret = getObject(arg0)[getObject(arg1)];
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_set_f975102236d3c502 = function() {
      return logError(function(arg0, arg1, arg2) {
        getObject(arg0)[takeObject(arg1)] = takeObject(arg2);
      }, arguments);
    };
    imports.wbg.__wbg_new_a220cf903aa02ca2 = function() {
      return logError(function() {
        const ret = new Array();
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_get_3baa728f9d58d3f6 = function() {
      return logError(function(arg0, arg1) {
        const ret = getObject(arg0)[arg1 >>> 0];
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_set_673dda6c73d19609 = function() {
      return logError(function(arg0, arg1, arg2) {
        getObject(arg0)[arg1 >>> 0] = takeObject(arg2);
      }, arguments);
    };
    imports.wbg.__wbg_from_0791d740a9d37830 = function() {
      return logError(function(arg0) {
        const ret = Array.from(getObject(arg0));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_isArray_8364a5371e9737d8 = function() {
      return logError(function(arg0) {
        const ret = Array.isArray(getObject(arg0));
        _assertBoolean(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_length_ae22078168b726f5 = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_instanceof_ArrayBuffer_61dfc3198373c902 = function() {
      return logError(function(arg0) {
        let result;
        try {
          result = getObject(arg0) instanceof ArrayBuffer;
        } catch (_) {
          result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_call_1084a111329e68ce = function() {
      return handleError(function(arg0, arg1) {
        const ret = getObject(arg0).call(getObject(arg1));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_new_8608a2b51a5f6737 = function() {
      return logError(function() {
        const ret = /* @__PURE__ */ new Map();
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_set_49185437f0ab06f8 = function() {
      return logError(function(arg0, arg1, arg2) {
        const ret = getObject(arg0).set(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_next_f9cb570345655b9a = function() {
      return handleError(function(arg0) {
        const ret = getObject(arg0).next();
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_next_de3e9db4440638b2 = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).next;
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_done_bfda7aa8f252b39f = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).done;
        _assertBoolean(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_value_6d39332ab4788d86 = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).value;
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_isSafeInteger_7f1ed56200d90674 = function() {
      return logError(function(arg0) {
        const ret = Number.isSafeInteger(getObject(arg0));
        _assertBoolean(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_entries_7a0e06255456ebcd = function() {
      return logError(function(arg0) {
        const ret = Object.entries(getObject(arg0));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_new_525245e2b9901204 = function() {
      return logError(function() {
        const ret = new Object();
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_fromCodePoint_ae875c4ff5f6a86b = function() {
      return handleError(function(arg0) {
        const ret = String.fromCodePoint(arg0 >>> 0);
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_iterator_888179a48810a9fe = function() {
      return logError(function() {
        const ret = Symbol.iterator;
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_instanceof_Uint8Array_247a91427532499e = function() {
      return logError(function(arg0) {
        let result;
        try {
          result = getObject(arg0) instanceof Uint8Array;
        } catch (_) {
          result = false;
        }
        const ret = result;
        _assertBoolean(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_new_ea1883e1e5e86686 = function() {
      return logError(function(arg0) {
        const ret = new Uint8Array(getObject(arg0));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_newwithbyteoffsetandlength_8a2cb9ca96b27ec9 = function() {
      return logError(function(arg0, arg1, arg2) {
        const ret = new Uint8Array(getObject(arg0), arg1 >>> 0, arg2 >>> 0);
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_buffer_0710d1b9dbe2eea6 = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_length_8339fcf5d8ecd12e = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).length;
        _assertNum(ret);
        return ret;
      }, arguments);
    };
    imports.wbg.__wbg_set_d1e79e2388520f18 = function() {
      return logError(function(arg0, arg1, arg2) {
        getObject(arg0).set(getObject(arg1), arg2 >>> 0);
      }, arguments);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
      const ret = typeof getObject(arg0) === "function";
      _assertBoolean(ret);
      return ret;
    };
    imports.wbg.__wbg_buffer_b7b08af79b0b0974 = function() {
      return logError(function(arg0) {
        const ret = getObject(arg0).buffer;
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbg_get_224d16597dbbfd96 = function() {
      return handleError(function(arg0, arg1) {
        const ret = Reflect.get(getObject(arg0), getObject(arg1));
        return addHeapObject(ret);
      }, arguments);
    };
    imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
      const ret = debugString(getObject(arg1));
      const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len1 = WASM_VECTOR_LEN;
      getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbindgen_bigint_get_as_i64 = function(arg0, arg1) {
      const v = getObject(arg1);
      const ret = typeof v === "bigint" ? v : void 0;
      if (!isLikeNone(ret)) {
        _assertBigInt(ret);
      }
      getDataViewMemory0().setBigInt64(arg0 + 8 * 1, isLikeNone(ret) ? BigInt(0) : ret, true);
      getDataViewMemory0().setInt32(arg0 + 4 * 0, !isLikeNone(ret), true);
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
      takeObject(arg0);
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
      throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_memory = function() {
      const ret = wasm.memory;
      return addHeapObject(ret);
    };
    return imports;
  }
  function __wbg_init_memory(imports, memory) {
  }
  function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    return wasm;
  }
  function initSync(module) {
    if (wasm !== void 0)
      return wasm;
    if (typeof module !== "undefined" && Object.getPrototypeOf(module) === Object.prototype)
      ({ module } = module);
    else
      console.warn("using deprecated parameters for `initSync()`; pass a single object instead");
    const imports = __wbg_get_imports();
    __wbg_init_memory(imports);
    if (!(module instanceof WebAssembly.Module)) {
      module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
  }
  async function __wbg_init(module_or_path) {
    if (wasm !== void 0)
      return wasm;
    if (typeof module_or_path !== "undefined" && Object.getPrototypeOf(module_or_path) === Object.prototype)
      ({ module_or_path } = module_or_path);
    else
      console.warn("using deprecated parameters for the initialization function; pass a single object instead");
    if (typeof module_or_path === "undefined" && typeof script_src !== "undefined") {
      module_or_path = script_src.replace(/\.js$/, "_bg.wasm");
    }
    const imports = __wbg_get_imports();
    if (typeof module_or_path === "string" || typeof Request === "function" && module_or_path instanceof Request || typeof URL === "function" && module_or_path instanceof URL) {
      module_or_path = fetch(module_or_path);
    }
    __wbg_init_memory(imports);
    const { instance, module } = await __wbg_load(await module_or_path, imports);
    return __wbg_finalize_init(instance, module);
  }
  wasm_bindgen = Object.assign(__wbg_init, { initSync }, __exports);
  const wasmArrayBuffer = import_alt_client.default.File.read("/client/rust_wasm_bg.wasm", "binary");
  initSync(wasmArrayBuffer);
  return wasm_bindgen;
};
var rust_wasm_bg_default = wasmLoader;

// js/altv_events.js
var import_alt_client4 = __toESM(require_alt_client(), 1);

// js/generation_id/client.js
var import_alt_client2 = __toESM(require_alt_client(), 1);
var U64_MAX = 2n ** 64n - 1n;
var current_generation_id = 1n;
import_alt_client2.default.on("baseObjectCreate", (base_object2) => {
  if (base_object2.isRemote)
    return;
  base_object2.generation_id = current_generation_id;
});
import_alt_client2.default.on("baseObjectRemove", () => {
  if (base_object.isRemote)
    return;
  if (current_generation_id + 1n > U64_MAX) {
    import_alt_client2.default.logError(
      "Client-side base object generation reached u64::MAX.\nNext base object will use non-unique generation.\nConsider opening issue in altv-rust repo: https://github.com/xxshady/altv-rust/issues."
    );
    current_generation_id = 0n;
  }
  current_generation_id += 1n;
});

// js/generation_id/helpers.js
var import_alt_client3 = __toESM(require_alt_client(), 1);
var GENERATION_ID_KEY = "&^#altv-rust";
function get_server_base_object_generation_id(base_object2) {
  let generation_id = base_object2.server_generation_id;
  if (generation_id != null)
    return generation_id;
  if (base_object2.getStreamSyncedMeta) {
    generation_id = base_object2.getStreamSyncedMeta(GENERATION_ID_KEY);
  } else if (base_object2.getSyncedMeta) {
    generation_id = base_object2.getSyncedMeta(GENERATION_ID_KEY);
  }
  import_alt_client3.default.Utils.assert(
    generation_id != null,
    `Failed to obtain generation id from server base object ${base_object2.constructor.name}`
  );
  base_object2.server_generation_id = generation_id;
  return generation_id;
}
function get_client_base_object_generation_id(base_object2) {
  const generation_id = base_object2.generation_id;
  import_alt_client3.default.Utils.assert(
    generation_id != null,
    `Failed to obtain generation id from client base object ${base_object2.constructor.name}`
  );
  return generation_id;
}

// js/helpers.js
function base_object_handle(base_object2) {
  return base_object2.isRemote ? {
    sdk_type: base_object2.type,
    is_remote: true,
    id: base_object2.remoteID,
    generation: get_server_base_object_generation_id(base_object2)
  } : {
    sdk_type: base_object2.type,
    is_remote: false,
    id: base_object2.id,
    generation: get_client_base_object_generation_id(base_object2)
  };
}

// js/altv_events.js
function enable_altv_event(resource, event_name) {
  const handlers = {
    serverStarted: () => {
      resource.call_export("on_altv_event", { serverStarted: {} });
    },
    consoleCommand: (name, ...args) => {
      resource.call_export("on_altv_event", { consoleCommand: { name, args } });
    },
    baseObjectCreate: (base_object2) => {
      if (base_object2.getStreamSyncedMeta) {
        import_alt_client4.default.log(
          "[baseObjectCreate] ignoring base object with stream synced meta:",
          base_object2.constructor.name
        );
        return;
      }
      resource.call_export("on_altv_event", {
        baseObjectCreate: {
          base_object: base_object_handle(base_object2)
        }
      });
    },
    baseObjectRemove: (base_object2) => {
      resource.call_export("on_altv_event", {
        baseObjectRemove: {
          base_object: base_object_handle(base_object2)
        }
      });
    },
    gameEntityCreate: (entity) => {
      resource.call_export("on_altv_event", {
        gameEntityCreate: {
          entity: base_object_handle(entity)
        }
      });
    },
    gameEntityDestroy: (entity) => {
      resource.call_export("on_altv_event", {
        gameEntityDestroy: {
          entity: base_object_handle(entity)
        }
      });
    },
    worldObjectStreamIn: (world_object) => {
      resource.call_export("on_altv_event", {
        worldObjectStreamIn: {
          world_object: base_object_handle(world_object)
        }
      });
    },
    worldObjectStreamOut: (world_object) => {
      resource.call_export("on_altv_event", {
        worldObjectStreamOut: {
          world_object: base_object_handle(world_object)
        }
      });
    }
  };
  const handler = handlers[event_name];
  if (!handler) {
    import_alt_client4.default.logError("unhandled event:", event_name);
    return;
  }
  resource.add_event_handler(event_name, handler);
}

// js/resource.js
var import_alt_client5 = __toESM(require_alt_client(), 1);
var Resource = class {
  exports = {};
  timers = [];
  event_handlers = /* @__PURE__ */ new Map();
  base_objects = /* @__PURE__ */ new Set();
  generic_local_event_handler = null;
  generic_remote_event_handler = null;
  constructor(exports) {
    this.exports = exports;
  }
  add_event_handler(event_name, handler) {
    import_alt_client5.default.Utils.assert(!this.event_handlers.has(event_name));
    import_alt_client5.default.on(event_name, handler);
    this.event_handlers.set(event_name, handler);
  }
  remove_event_handler(event_name) {
    const handler = this.event_handlers.get(event_name);
    import_alt_client5.default.Utils.assert(handler != null);
    import_alt_client5.default.off(event_name, handler);
    this.event_handlers.delete(event_name);
  }
  add_timer(timer) {
    this.timers.push(timer);
  }
  add_base_object(base_object2) {
    this.base_objects.add(base_object2);
  }
  call_export(name, ...args) {
    try {
      this.exports[name](...args);
    } catch (e) {
      import_alt_client5.default.logError(
        `Export call '${name}'`,
        "\n" + this.exports.get_current_panic_info(),
        "\n\n" + e.stack.split("\n").slice(1).join("\n")
      );
      this.drop();
    }
  }
  drop() {
    for (const timer of this.timers) {
      import_alt_client5.default.clearTimer(timer);
    }
    for (const [event_name, handler] of this.event_handlers) {
      import_alt_client5.default.off(event_name, handler);
    }
    for (const base_object2 of this.base_objects) {
      base_object2.destroy();
    }
    import_alt_client5.default.off(this.generic_local_event_handler);
    import_alt_client5.default.offServer(this.generic_remote_event_handler);
  }
};

// js/script_events.js
var import_alt_client6 = __toESM(require_alt_client(), 1);
function init(resource) {
  resource.generic_local_event_handler = (event_name, ...args) => {
    resource.call_export("on_script_event", {
      source: 0,
      name: event_name,
      args
    });
  };
  import_alt_client6.default.on(resource.generic_local_event_handler);
  resource.generic_remote_event_handler = (event_name, ...args) => {
    resource.call_export("on_script_event", {
      source: 1,
      name: event_name,
      args
    });
  };
  import_alt_client6.default.onServer(resource.generic_remote_event_handler);
}

// js/sourcemap.js
var import_alt_client7 = __toESM(require_alt_client(), 1);
var import_source_map = __toESM(require_source_map(), 1);
async function init2() {
  const sourcemap = import_alt_client7.default.File.read("/client/wasm.map", "utf-8");
  const wasmMappings = import_alt_client7.default.File.read("/client/mappings.wasm", "binary");
  import_source_map.SourceMapConsumer.initialize({ "lib/mappings.wasm": wasmMappings });
  const consumer = await new import_source_map.SourceMapConsumer(sourcemap);
  Error.prepareStackTrace = (err, frames) => {
    return err.stack.split("\n").map((frameStr, idx) => {
      if (!frameStr.includes("wasm://"))
        return frameStr;
      const frame = frames[idx - 1];
      import_alt_client7.default.Utils.assert(frame != null);
      const column = frame.getColumnNumber();
      import_alt_client7.default.Utils.assert(column != null);
      const original = consumer.originalPositionFor({ line: 1, column: column - 1 });
      if (original?.line == null) {
        return frameStr;
      }
      return frameStr.replace(
        /\(wasm:\/\/.*\)/,
        `(${original.source}:${original.line}:${original.column + 1})`
      );
    }).join("\n");
  };
}

// js/init_created_base_objects.js
var import_alt_client8 = __toESM(require_alt_client(), 1);
function init_created_base_objects(resource) {
  import_alt_client8.default.Vehicle.streamedIn.forEach((v) => {
    resource.call_export("on_altv_event", {
      gameEntityCreate: {
        entity: base_object_handle(v)
      }
    });
  });
  import_alt_client8.default.Player.streamedIn.forEach((v) => {
    resource.call_export("on_altv_event", {
      gameEntityCreate: {
        entity: base_object_handle(v)
      }
    });
  });
}

// js/main.js
async function main() {
  Error.stackTraceLimit = 100;
  await init2();
  let resource_instance;
  const exports = rust_wasm_bg_default({
    log_info(string) {
      import_alt_client9.default.log("[rust]", string);
    },
    log_warn(string) {
      import_alt_client9.default.logWarning("[rust]", string);
    },
    log_error(string) {
      import_alt_client9.default.logError("[rust]", string);
    },
    enable_altv_event(event_name) {
      enable_altv_event(resource_instance, event_name);
    },
    disable_altv_event(event_name) {
      resource_instance.remove_event_handler(event_name);
    },
    get_base_object_ref(sdk_type, is_remote, id) {
      if (is_remote) {
        return import_alt_client9.default.BaseObject.getByRemoteID(sdk_type, id);
      } else {
        return import_alt_client9.default.BaseObject.getByID(sdk_type, id);
      }
    },
    emit_local_event_rust(event_name, buffer) {
      console.log("emit_local_event_rust", { event_name, buffer });
      import_alt_client9.default.emit(event_name, buffer);
    },
    emit_local_event_js(event_name, args) {
      import_alt_client9.default.emit(event_name, ...args);
    },
    get_streamed_in_players() {
      return import_alt_client9.default.Player.streamedIn.map((p) => ({
        id: p.id,
        generation: get_server_base_object_generation_id(p)
      }));
    },
    get_streamed_in_vehicles() {
      return import_alt_client9.default.Vehicle.streamedIn.map((v) => ({
        id: v.id,
        generation: get_server_base_object_generation_id(v)
      }));
    },
    get_net_time() {
      return import_alt_client9.default.getNetTime();
    },
    get_base_object_raw_handle(js_ref) {
      return base_object_handle(js_ref);
    },
    get_local_player() {
      return import_alt_client9.default.Player.local;
    },
    is_local_player(base_object2) {
      return import_alt_client9.default.Player.local === base_object2;
    },
    BaseObject: import_alt_client9.default.BaseObject
  });
  resource_instance = new Resource(exports);
  init(resource_instance);
  resource_instance.call_export("main");
  init_created_base_objects(resource_instance);
  resource_instance.add_timer(import_alt_client9.default.everyTick(() => {
    resource_instance.call_export("on_every_tick");
  }));
  resource_instance.call_export("test_script_events");
}
main().catch(import_alt_client9.default.logError);

// ------------------- altv-esbuild footer -------------------

// ------------------- altv-esbuild footer -------------------

