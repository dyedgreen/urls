var c,S,K,Y,P={},$=[],Se=/acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;function C(e,t){for(var n in t)e[n]=t[n];return e}function Z(e){var t=e.parentNode;t&&t.removeChild(e)}function l(e,t,n){var r,_,o,s=arguments,f={};for(o in t)o=="key"?r=t[o]:o=="ref"?_=t[o]:f[o]=t[o];if(arguments.length>3)for(n=[n],o=3;o<arguments.length;o++)n.push(s[o]);if(n!=null&&(f.children=n),typeof e=="function"&&e.defaultProps!=null)for(o in e.defaultProps)f[o]===void 0&&(f[o]=e.defaultProps[o]);return H(e,f,r,_,null)}function H(e,t,n,r,_){var o={type:e,props:t,key:n,ref:r,__k:null,__:null,__b:0,__e:null,__d:void 0,__c:null,__h:null,constructor:void 0,__v:_==null?++c.__v:_};return c.vnode!=null&&c.vnode(o),o}function M(e){return e.children}function j(e,t){this.props=e,this.context=t}function E(e,t){if(t==null)return e.__?E(e.__,e.__.__k.indexOf(e)+1):null;for(var n;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null)return n.__e;return typeof e.type=="function"?E(e):null}function ee(e){var t,n;if((e=e.__)!=null&&e.__c!=null){for(e.__e=e.__c.base=null,t=0;t<e.__k.length;t++)if((n=e.__k[t])!=null&&n.__e!=null){e.__e=e.__c.base=n.__e;break}return ee(e)}}function te(e){(!e.__d&&(e.__d=!0)&&S.push(e)&&!D.__r++||Y!==c.debounceRendering)&&((Y=c.debounceRendering)||K)(D)}function D(){for(var e;D.__r=S.length;)e=S.sort(function(t,n){return t.__v.__b-n.__v.__b}),S=[],e.some(function(t){var n,r,_,o,s,f;t.__d&&(s=(o=(n=t).__v).__e,(f=n.__P)&&(r=[],(_=C({},o)).__v=o.__v+1,q(f,o,_,n.__n,f.ownerSVGElement!==void 0,o.__h!=null?[s]:null,r,s==null?E(o):s,o.__h),ne(r,o),o.__e!=s&&ee(o)))})}function le(e,t,n,r,_,o,s,f,p,d){var i,m,u,a,v,h,g,y=r&&r.__k||$,b=y.length;for(n.__k=[],i=0;i<t.length;i++)if((a=n.__k[i]=(a=t[i])==null||typeof a=="boolean"?null:typeof a=="string"||typeof a=="number"||typeof a=="bigint"?H(null,a,null,null,a):Array.isArray(a)?H(M,{children:a},null,null,null):a.__b>0?H(a.type,a.props,a.key,null,a.__v):a)!=null){if(a.__=n,a.__b=n.__b+1,(u=y[i])===null||u&&a.key==u.key&&a.type===u.type)y[i]=void 0;else for(m=0;m<b;m++){if((u=y[m])&&a.key==u.key&&a.type===u.type){y[m]=void 0;break}u=null}q(e,a,u=u||P,_,o,s,f,p,d),v=a.__e,(m=a.ref)&&u.ref!=m&&(g||(g=[]),u.ref&&g.push(u.ref,null,a),g.push(m,a.__c||v,a)),v!=null?(h==null&&(h=v),typeof a.type=="function"&&a.__k!=null&&a.__k===u.__k?a.__d=p=oe(a,p,e):p=re(e,a,u,y,v,p),d||n.type!=="option"?typeof n.type=="function"&&(n.__d=p):e.value=""):p&&u.__e==p&&p.parentNode!=e&&(p=E(u))}for(n.__e=h,i=b;i--;)y[i]!=null&&(typeof n.type=="function"&&y[i].__e!=null&&y[i].__e==n.__d&&(n.__d=E(r,i+1)),_e(y[i],y[i]));if(g)for(i=0;i<g.length;i++)ie(g[i],g[++i],g[++i])}function oe(e,t,n){var r,_;for(r=0;r<e.__k.length;r++)(_=e.__k[r])&&(_.__=e,t=typeof _.type=="function"?oe(_,t,n):re(n,_,_,e.__k,_.__e,t));return t}function re(e,t,n,r,_,o){var s,f,p;if(t.__d!==void 0)s=t.__d,t.__d=void 0;else if(n==null||_!=o||_.parentNode==null)e:if(o==null||o.parentNode!==e)e.appendChild(_),s=null;else{for(f=o,p=0;(f=f.nextSibling)&&p<r.length;p+=2)if(f==_)break e;e.insertBefore(_,o),s=o}return s!==void 0?s:_.nextSibling}function Ee(e,t,n,r,_){var o;for(o in n)o==="children"||o==="key"||o in t||F(e,o,null,n[o],r);for(o in t)_&&typeof t[o]!="function"||o==="children"||o==="key"||o==="value"||o==="checked"||n[o]===t[o]||F(e,o,t[o],n[o],r)}function ae(e,t,n){t[0]==="-"?e.setProperty(t,n):e[t]=n==null?"":typeof n!="number"||Se.test(t)?n:n+"px"}function F(e,t,n,r,_){var o;e:if(t==="style")if(typeof n=="string")e.style.cssText=n;else{if(typeof r=="string"&&(e.style.cssText=r=""),r)for(t in r)n&&t in n||ae(e.style,t,"");if(n)for(t in n)r&&n[t]===r[t]||ae(e.style,t,n[t])}else if(t[0]==="o"&&t[1]==="n")o=t!==(t=t.replace(/Capture$/,"")),t=t.toLowerCase()in e?t.toLowerCase().slice(2):t.slice(2),e.l||(e.l={}),e.l[t+o]=n,n?r||e.addEventListener(t,o?ce:se,o):e.removeEventListener(t,o?ce:se,o);else if(t!=="dangerouslySetInnerHTML"){if(_)t=t.replace(/xlink[H:h]/,"h").replace(/sName$/,"s");else if(t!=="href"&&t!=="list"&&t!=="form"&&t!=="tabIndex"&&t!=="download"&&t in e)try{e[t]=n==null?"":n;break e}catch(s){}typeof n=="function"||(n!=null&&(n!==!1||t[0]==="a"&&t[1]==="r")?e.setAttribute(t,n):e.removeAttribute(t))}}function se(e){this.l[e.type+!1](c.event?c.event(e):e)}function ce(e){this.l[e.type+!0](c.event?c.event(e):e)}function q(e,t,n,r,_,o,s,f,p){var d,i,m,u,a,v,h,g,y,b,I,x=t.type;if(t.constructor!==void 0)return null;n.__h!=null&&(p=n.__h,f=t.__e=n.__e,t.__h=null,o=[f]),(d=c.__b)&&d(t);try{e:if(typeof x=="function"){if(g=t.props,y=(d=x.contextType)&&r[d.__c],b=d?y?y.props.value:d.__:r,n.__c?h=(i=t.__c=n.__c).__=i.__E:("prototype"in x&&x.prototype.render?t.__c=i=new x(g,b):(t.__c=i=new j(g,b),i.constructor=x,i.render=Ae),y&&y.sub(i),i.props=g,i.state||(i.state={}),i.context=b,i.__n=r,m=i.__d=!0,i.__h=[]),i.__s==null&&(i.__s=i.state),x.getDerivedStateFromProps!=null&&(i.__s==i.state&&(i.__s=C({},i.__s)),C(i.__s,x.getDerivedStateFromProps(g,i.__s))),u=i.props,a=i.state,m)x.getDerivedStateFromProps==null&&i.componentWillMount!=null&&i.componentWillMount(),i.componentDidMount!=null&&i.__h.push(i.componentDidMount);else{if(x.getDerivedStateFromProps==null&&g!==u&&i.componentWillReceiveProps!=null&&i.componentWillReceiveProps(g,b),!i.__e&&i.shouldComponentUpdate!=null&&i.shouldComponentUpdate(g,i.__s,b)===!1||t.__v===n.__v){i.props=g,i.state=i.__s,t.__v!==n.__v&&(i.__d=!1),i.__v=t,t.__e=n.__e,t.__k=n.__k,t.__k.forEach(function(T){T&&(T.__=t)}),i.__h.length&&s.push(i);break e}i.componentWillUpdate!=null&&i.componentWillUpdate(g,i.__s,b),i.componentDidUpdate!=null&&i.__h.push(function(){i.componentDidUpdate(u,a,v)})}i.context=b,i.props=g,i.state=i.__s,(d=c.__r)&&d(t),i.__d=!1,i.__v=t,i.__P=e,d=i.render(i.props,i.state,i.context),i.state=i.__s,i.getChildContext!=null&&(r=C(C({},r),i.getChildContext())),m||i.getSnapshotBeforeUpdate==null||(v=i.getSnapshotBeforeUpdate(u,a)),I=d!=null&&d.type===M&&d.key==null?d.props.children:d,le(e,Array.isArray(I)?I:[I],t,n,r,_,o,s,f,p),i.base=t.__e,t.__h=null,i.__h.length&&s.push(i),h&&(i.__E=i.__=null),i.__e=!1}else o==null&&t.__v===n.__v?(t.__k=n.__k,t.__e=n.__e):t.__e=Ne(n.__e,t,n,r,_,o,s,p);(d=c.diffed)&&d(t)}catch(T){t.__v=null,(p||o!=null)&&(t.__e=f,t.__h=!!p,o[o.indexOf(f)]=null),c.__e(T,t,n)}}function ne(e,t){c.__c&&c.__c(t,e),e.some(function(n){try{e=n.__h,n.__h=[],e.some(function(r){r.call(n)})}catch(r){c.__e(r,n.__v)}})}function Ne(e,t,n,r,_,o,s,f){var p,d,i,m,u=n.props,a=t.props,v=t.type,h=0;if(v==="svg"&&(_=!0),o!=null){for(;h<o.length;h++)if((p=o[h])&&(p===e||(v?p.localName==v:p.nodeType==3))){e=p,o[h]=null;break}}if(e==null){if(v===null)return document.createTextNode(a);e=_?document.createElementNS("http://www.w3.org/2000/svg",v):document.createElement(v,a.is&&a),o=null,f=!1}if(v===null)u===a||f&&e.data===a||(e.data=a);else{if(o=o&&$.slice.call(e.childNodes),d=(u=n.props||P).dangerouslySetInnerHTML,i=a.dangerouslySetInnerHTML,!f){if(o!=null)for(u={},m=0;m<e.attributes.length;m++)u[e.attributes[m].name]=e.attributes[m].value;(i||d)&&(i&&(d&&i.__html==d.__html||i.__html===e.innerHTML)||(e.innerHTML=i&&i.__html||""))}if(Ee(e,a,u,_,f),i)t.__k=[];else if(h=t.props.children,le(e,Array.isArray(h)?h:[h],t,n,r,_&&v!=="foreignObject",o,s,e.firstChild,f),o!=null)for(h=o.length;h--;)o[h]!=null&&Z(o[h]);f||("value"in a&&(h=a.value)!==void 0&&(h!==e.value||v==="progress"&&!h)&&F(e,"value",h,u.value,!1),"checked"in a&&(h=a.checked)!==void 0&&h!==e.checked&&F(e,"checked",h,u.checked,!1))}return e}function ie(e,t,n){try{typeof e=="function"?e(t):e.current=t}catch(r){c.__e(r,n)}}function _e(e,t,n){var r,_,o;if(c.unmount&&c.unmount(e),(r=e.ref)&&(r.current&&r.current!==e.__e||ie(r,null,t)),n||typeof e.type=="function"||(n=(_=e.__e)!=null),e.__e=e.__d=void 0,(r=e.__c)!=null){if(r.componentWillUnmount)try{r.componentWillUnmount()}catch(s){c.__e(s,t)}r.base=r.__P=null}if(r=e.__k)for(o=0;o<r.length;o++)r[o]&&_e(r[o],t,n);_!=null&&Z(_)}function Ae(e,t,n){return this.constructor(e,n)}function W(e,t,n){var r,_,o;c.__&&c.__(e,t),_=(r=typeof n=="function")?null:n&&n.__k||t.__k,o=[],q(t,e=(!r&&n||t).__k=l(M,null,[e]),_||P,P,t.ownerSVGElement!==void 0,!r&&n?[n]:_?null:t.firstChild?$.slice.call(t.childNodes):null,o,!r&&n?n:_?_.__e:t.firstChild,r),ne(o,e)}c={__e:function(e,t){for(var n,r,_;t=t.__;)if((n=t.__c)&&!n.__)try{if((r=n.constructor)&&r.getDerivedStateFromError!=null&&(n.setState(r.getDerivedStateFromError(e)),_=n.__d),n.componentDidCatch!=null&&(n.componentDidCatch(e),_=n.__d),_)return n.__E=n}catch(o){e=o}throw e},__v:0},j.prototype.setState=function(e,t){var n;n=this.__s!=null&&this.__s!==this.state?this.__s:this.__s=C({},this.state),typeof e=="function"&&(e=e(C({},n),this.props)),e&&C(n,e),e!=null&&this.__v&&(t&&this.__h.push(t),te(this))},j.prototype.forceUpdate=function(e){this.__v&&(this.__e=!0,e&&this.__h.push(e),te(this))},j.prototype.render=M,S=[],K=typeof Promise=="function"?Promise.prototype.then.bind(Promise.resolve()):setTimeout,D.__r=0;var R,w,ue,z=0,Q=[],fe=c.__b,pe=c.__r,de=c.diffed,he=c.__c,me=c.unmount;function ge(e,t){c.__h&&c.__h(w,e,z||t),z=0;var n=w.__H||(w.__H={__:[],__h:[]});return e>=n.__.length&&n.__.push({}),n.__[e]}function k(e){return z=1,Ie(ve,e)}function Ie(e,t,n){var r=ge(R++,2);return r.t=e,r.__c||(r.__=[n?n(t):ve(void 0,t),function(_){var o=r.t(r.__[0],_);r.__[0]!==o&&(r.__=[o,r.__[1]],r.__c.setState({}))}],r.__c=w),r.__}function V(e,t){var n=ge(R++,3);!c.__s&&Te(n.__H,t)&&(n.__=e,n.__H=t,w.__H.__h.push(n))}function Pe(){Q.forEach(function(e){if(e.__P)try{e.__H.__h.forEach(B),e.__H.__h.forEach(G),e.__H.__h=[]}catch(t){e.__H.__h=[],c.__e(t,e.__v)}}),Q=[]}c.__b=function(e){w=null,fe&&fe(e)},c.__r=function(e){pe&&pe(e),R=0;var t=(w=e.__c).__H;t&&(t.__h.forEach(B),t.__h.forEach(G),t.__h=[])},c.diffed=function(e){de&&de(e);var t=e.__c;t&&t.__H&&t.__H.__h.length&&(Q.push(t)!==1&&ue===c.requestAnimationFrame||((ue=c.requestAnimationFrame)||function(n){var r,_=function(){clearTimeout(o),ye&&cancelAnimationFrame(r),setTimeout(n)},o=setTimeout(_,100);ye&&(r=requestAnimationFrame(_))})(Pe)),w=void 0},c.__c=function(e,t){t.some(function(n){try{n.__h.forEach(B),n.__h=n.__h.filter(function(r){return!r.__||G(r)})}catch(r){t.some(function(_){_.__h&&(_.__h=[])}),t=[],c.__e(r,n.__v)}}),he&&he(e,t)},c.unmount=function(e){me&&me(e);var t=e.__c;if(t&&t.__H)try{t.__H.__.forEach(B)}catch(n){c.__e(n,t.__v)}};var ye=typeof requestAnimationFrame=="function";function B(e){var t=w;typeof e.__c=="function"&&e.__c(),w=t}function G(e){var t=w;e.__c=e.__(),w=t}function Te(e,t){return!e||e.length!==t.length||t.some(function(n,r){return n!==e[r]})}function ve(e,t){return typeof t=="function"?t(e):t}async function ke(e,t){let n=window.__xsrf_token;return await(await fetch("/graphql",{method:"POST",credentials:"include",headers:{"Content-Type":"application/json","X-XSRF-Token":n},body:JSON.stringify({query:e,variables:t})})).json()}function O([e]){return e}function we(e,t){let[n,r]=k(null),[_,o]=k(!0),[s,f]=k(null);return V(()=>{ke(e,t).then(({data:p,errors:d})=>{d!=null?f(d):(r(p),o(!1))}).catch(p=>f(p))},[e,t]),V(()=>{if(s!=null)throw s},[s]),{data:n,loading:_}}function be(e,{onCommit:t,onError:n}){let[r,_]=k(!1),[o,s]=k(void 0);if(o!==void 0)throw o;return{commit:p=>{(async()=>{_(!0);let{data:d,errors:i}=await ke(e,p);_(!1),i!=null?typeof n=="function"?n(i):s(error):t(d)})()},inFlight:r}}function N({size:e="small"}){return l("svg",{xmlns:"http://www.w3.org/2000/svg",class:e==="large"?"h-8 w-8":"h-4 w-4",viewBox:"0 0 24 24",stroke:"none",fill:"currentColor"},l("rect",{x:"10",y:"0",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 6.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.125s",repeatCount:"indefinite"})),l("rect",{x:"16",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.25s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(17.5, 17.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.375s",repeatCount:"indefinite"})),l("rect",{x:"10",y:"16",width:"4",height:"8",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.5s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 17.5) rotate(45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.625s",repeatCount:"indefinite"})),l("rect",{x:"0",y:"10",width:"8",height:"4",rx:"2",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.75s",repeatCount:"indefinite"})),l("rect",{x:"-2",y:"-4",width:"4",height:"8",rx:"2",transform:"translate(6.5, 6.5) rotate(-45)",opacity:"0"},l("animate",{attributeName:"opacity",from:"1",to:"0",dur:"1s",begin:"0.875s",repeatCount:"indefinite"})))}function A({title:e,onClick:t,disabled:n=!1,loading:r=!1,type:_="default",style:o}){let s=`h-8 px-2 flex space-x-2 justify-center items-center rounded-md font-bold ${o!=null?o:""}`;switch(_){case"flat":s+=" bg-white text-black",n||(s+=" hover:bg-gray-100");break;case"default":default:s+=" bg-blue-500 text-white",n||(s+=" hover:bg-blue-400")}return n===!0&&(s+=" opacity-80 cursor-not-allowed"),l("button",{class:s,onClick:n===!0?void 0:t},l("div",null,e),r===!0&&l(N,null))}function He(){window.location.href=window.location.href}function Me({error:e}){return l("div",{class:"w-full flex justify-center p-8"},l("div",{class:"w-full max-w-3xl bg-red-500 rounded-lg p-4"},l("h1",{class:"text-white text-xl font-semibold"},"Error"),l("p",{class:"text-red-100"},"Sorry, something went wrong."),l("pre",{class:"text-white bg-red-900 rounded-lg p-4 mt-2 whitespace-pre-wrap"},JSON.stringify(e)),l(A,{title:"Reload Page",onClick:He,type:"flat",style:"w-full mt-2"})))}var J=class extends j{constructor(){super();this.state={error:!1}}componentDidCatch(t){console.error(t),this.setState({error:t})}render({children:t},{error:n}){return n!==!1?l(Me,{error:n}):t}},xe=J;var De=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M5 15l7-7 7 7"})),Fe=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M19 9l-7 7-7-7"}));function X({title:e,children:t,initiallyExpanded:n=!0}){let[r,_]=k(n);return l("div",{class:"w-full mt-2 rounded-md bg-gray-100 dark:bg-gray-700"},l("button",{class:"w-full p-2 flex justify-between items-center rounded-md hover:bg-gray-200 dark:hover:bg-gray-600",onClick:()=>_(!r)},l("h1",{class:"text-lg font-semibold"},e),r?De:Fe),l("div",{class:"w-full px-2 pb-2",style:{display:r?void 0:"none"}},t))}function L({value:e,onChange:t,label:n,placeholder:r,style:_}){let[o]=k(`input-id-${Math.floor(Math.random()*1e4)}`);return l("div",{class:`w-full ${_!=null?_:""}`},l("label",{class:"text-gray-500 italic",for:o},n),l("input",{class:"w-full p-2 text-md rounded-md bg-gray-200 dark:bg-gray-600 text-black dark:text-white",id:o,type:"text",value:e,onInput:s=>typeof t=="function"&&t(s.target.value),placeholder:r}))}var Be=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M7 8h10M7 12h4m1 8l-4-4H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-3l-4 4z"})),Ce=l("svg",{xmlns:"http://www.w3.org/2000/svg",class:"h-6 w-6",fill:"none",viewBox:"0 0 24 24",stroke:"currentColor"},l("path",{"stroke-linecap":"round","stroke-linejoin":"round","stroke-width":"2",d:"M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"}));function U({type:e="info",message:t,style:n}){let r=`flex align-start p-3 rounded-md block font-semibold ${n!=null?n:""}`,_;switch(e){case"error":r+=" bg-red-200 text-red-800 dark:bg-red-800 dark:text-red-200",_=Ce;break;case"warning":r+=" bg-yellow-200 text-yellow-800 dark:bg-yellow-800 dark:text-yellow-200",_=Ce;break;case"info":default:r+=" bg-blue-200 text-blue-800 dark:bg-blue-800 dark:text-blue-200",_=Be}return l("div",{class:r},l("div",{class:"mr-2"},_),l("p",{class:"leading-snug"},t))}function je({}){let[e,t]=k(""),[n,r]=k(""),[_,o]=k(null),[s,f]=k(null),{commit:p,inFlight:d}=be(O`
    mutation ChangeEmailMutation($email: String!) {
      updateEmail(email: $email) {
        id
        email
      }
    }
  `,{onCommit:({updateEmail:u})=>{t(""),r(""),o(null),f(`Email changed to ${u==null?void 0:u.email}`)},onError:([{message:u}])=>{o(`Failed to change email: ${u}`),f(null)}}),i=!d&&e.trim().length>0&&e===n,m=u=>{u.preventDefault(),i&&p({email:e.trim()})};return l("form",{onSubmit:m},_&&l(U,{message:_,type:"error",style:"mb-2"}),s&&l(U,{message:s,style:"mb-2"}),l(L,{label:"New email address",placeholder:"ada.lovelace@urls.fyi",value:e,onChange:t,style:"mb-2"}),l(L,{label:"Confirm email address",placeholder:"ada.lovelace@urls.fyi",value:n,onChange:r,style:"mb-2"}),l(A,{title:"Update",onClick:m,disabled:!i,loading:d}))}function Oe(){var n,r;let{data:e,loading:t}=we(O`
    query AccountQuery {
      viewer {
        id
        user {
          id
          name
        }
      }
    }
  `);return l("div",{class:"w-full flex justify-center p-8"},t?l(N,{size:"large"}):l("div",{class:"w-full max-w-screen-md bg-white dark:bg-gray-800 shadow rounded-lg p-4"},l("h1",{class:"text-2xl font-semibold"},"Account Settings"),l("h2",{class:"text-xl text-gray-500 mb-4"},"Welcome back ",(r=(n=e==null?void 0:e.viewer)==null?void 0:n.user)==null?void 0:r.name),l(X,{title:"Invite a friend"},"TODO"),l(X,{title:"Change email",initiallyExpanded:!1},l(je,null))))}W(l(xe,null,l(Oe,null)),document.getElementById("account"));
//# sourceMappingURL=account.js.map
