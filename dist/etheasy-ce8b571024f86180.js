let V=`string`,U=1,R=`utf-8`,Y=3,X=`Object`,P=null,_=4,Q=`undefined`,N=128,W=`function`,T=0,M=Array,S=Error,Z=FinalizationRegistry,$=Object,O=undefined;var F=((a,b)=>{a=a>>>T;const c=E();const d=c.subarray(a/_,a/_+ b);const e=[];for(let a=T;a<d.length;a++){e.push(f(d[a]))};return e});var J=((b,c)=>{a=b.exports;L.__wbindgen_wasm_module=c;q=P;D=P;h=P;a.__wbindgen_start();return a});var E=(()=>{if(D===P||D.byteLength===T){D=new Uint32Array(a.memory.buffer)};return D});var y=((b,c,d,e)=>{const f={a:b,b:c,cnt:U,dtor:d};const g=(...b)=>{f.cnt++;try{return e(f.a,f.b,...b)}finally{if(--f.cnt===T){a.__wbindgen_export_2.get(f.dtor)(f.a,f.b);f.a=T;s.unregister(f)}}};g.original=f;s.register(g,f,f);return g});var i=(()=>{if(h===P||h.byteLength===T){h=new Uint8Array(a.memory.buffer)};return h});var G=(async(a,b)=>{if(typeof Response===W&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===W){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var K=(b=>{if(a!==O)return a;const c=H();I(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return J(d,b)});function C(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(k(b))}}var B=(a=>a===O||a===P);var j=((a,b)=>{a=a>>>T;return g.decode(i().subarray(a,a+ b))});var A=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h90e3b3e9622906f1(b,c,k(d))});var p=((a,b,c)=>{if(c===O){const c=n.encode(a);const d=b(c.length,U)>>>T;i().subarray(d,d+ c.length).set(c);m=c.length;return d};let d=a.length;let e=b(d,U)>>>T;const f=i();let g=T;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==T){a=a.slice(g)};e=c(e,d,d=g+ a.length*Y,U)>>>T;const b=i().subarray(e+ g,e+ d);const f=o(a,b);g+=f.written;e=c(e,d,g,U)>>>T};m=g;return e});var k=(a=>{if(d===b.length)b.push(b.length+ U);const c=d;d=b[c];b[c]=a;return c});var c=(a=>b[a]);var w=((c,d,e)=>{try{a.wasm_bindgen__convert__closures__invoke1_mut_ref__h4734736d7ae9e79a(c,d,v(e))}finally{b[u++]=O}});var r=(()=>{if(q===P||q.byteLength===T){q=new Int32Array(a.memory.buffer)};return q});var v=(a=>{if(u==U)throw new S(`out of js stack`);b[--u]=a;return u});var L=(async(b)=>{if(a!==O)return a;if(typeof b===Q){b=new URL(`etheasy_bg.wasm`,import.meta.url)};const c=H();if(typeof b===V||typeof Request===W&&b instanceof Request||typeof URL===W&&b instanceof URL){b=fetch(b)};I(c);const {instance:d,module:e}=await G(await b,c);return J(d,e)});var e=(a=>{if(a<132)return;b[a]=d;d=a});var z=((c,d,e)=>{try{a._dyn_core__ops__function__Fn___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h664ff4e3f7aa372c(c,d,v(e))}finally{b[u++]=O}});var I=((a,b)=>{});var x=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h661aab9422a0bbb6(b,c)});var H=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_drop_ref=(a=>{f(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=j(a,b);return k(c)});b.wbg.__wbindgen_cb_drop=(a=>{const b=f(a).original;if(b.cnt--==U){b.a=T;return !0};const c=!1;return c});b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return k(b)});b.wbg.__wbg_listenerid_6dcf1c62b7b7de58=((a,b)=>{const d=c(b).__yew_listener_id;r()[a/_+ U]=B(d)?T:d;r()[a/_+ T]=!B(d)});b.wbg.__wbg_setlistenerid_f2e783343fa0cec1=((a,b)=>{c(a).__yew_listener_id=b>>>T});b.wbg.__wbg_subtreeid_e80a1798fee782f9=((a,b)=>{const d=c(b).__yew_subtree_id;r()[a/_+ U]=B(d)?T:d;r()[a/_+ T]=!B(d)});b.wbg.__wbg_setsubtreeid_e1fab6b578c800cf=((a,b)=>{c(a).__yew_subtree_id=b>>>T});b.wbg.__wbg_cachekey_b81c1aacc6a0645c=((a,b)=>{const d=c(b).__yew_subtree_cache_key;r()[a/_+ U]=B(d)?T:d;r()[a/_+ T]=!B(d)});b.wbg.__wbg_setcachekey_75bcd45312087529=((a,b)=>{c(a).__yew_subtree_cache_key=b>>>T});b.wbg.__wbg_new_abda76e883ba8a5f=(()=>{const a=new S();return k(a)});b.wbg.__wbg_stack_658279fe44541cf6=((b,d)=>{const e=c(d).stack;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbg_error_f851667af71bcfc6=((b,c)=>{let d;let e;try{d=b;e=c;console.error(j(b,c))}finally{a.__wbindgen_free(d,e,U)}});b.wbg.__wbg_setTimeout_7d81d052875b0f4f=function(){return C(((a,b)=>{const d=setTimeout(c(a),b);return k(d)}),arguments)};b.wbg.__wbg_clearTimeout_541ac0980ffcef74=(a=>{const b=clearTimeout(f(a));return k(b)});b.wbg.__wbg_queueMicrotask_481971b0d87f3dd4=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_3cbae2ec6b6cd3d6=(a=>{const b=c(a).queueMicrotask;return k(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===W;return b});b.wbg.__wbg_error_a526fb08a0205972=((b,c)=>{var d=F(b,c).slice();a.__wbindgen_free(b,c*_,_);console.error(...d)});b.wbg.__wbg_log_7c3433e130418e14=((b,c)=>{var d=F(b,c).slice();a.__wbindgen_free(b,c*_,_);console.log(...d)});b.wbg.__wbg_body_edb1908d3ceff3a1=(a=>{const b=c(a).body;return B(b)?T:k(b)});b.wbg.__wbg_createElement_8bae7856a4bb7411=function(){return C(((a,b,d)=>{const e=c(a).createElement(j(b,d));return k(e)}),arguments)};b.wbg.__wbg_createElementNS_556a62fb298be5a2=function(){return C(((a,b,d,e,f)=>{const g=c(a).createElementNS(b===T?O:j(b,d),j(e,f));return k(g)}),arguments)};b.wbg.__wbg_createTextNode_0c38fd80a5b2284d=((a,b,d)=>{const e=c(a).createTextNode(j(b,d));return k(e)});b.wbg.__wbg_instanceof_Window_f401953a2cf86220=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5100775d18896c16=(a=>{const b=c(a).document;return B(b)?T:k(b)});b.wbg.__wbg_instanceof_Element_6945fc210db80ea9=(a=>{let b;try{b=c(a) instanceof Element}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_namespaceURI_5235ee79fd5f6781=((b,d)=>{const e=c(d).namespaceURI;var f=B(e)?T:p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbg_clientHeight_d24efa25aa66e844=(a=>{const b=c(a).clientHeight;return b});b.wbg.__wbg_setinnerHTML_26d69b59e1af99c7=((a,b,d)=>{c(a).innerHTML=j(b,d)});b.wbg.__wbg_outerHTML_e073aa84e7bc1eaf=((b,d)=>{const e=c(d).outerHTML;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbg_removeAttribute_1b10a06ae98ebbd1=function(){return C(((a,b,d)=>{c(a).removeAttribute(j(b,d))}),arguments)};b.wbg.__wbg_scrollIntoView_0c1a31f3d0dce6ae=(a=>{c(a).scrollIntoView()});b.wbg.__wbg_setAttribute_3c9f6c303b696daa=function(){return C(((a,b,d,e,f)=>{c(a).setAttribute(j(b,d),j(e,f))}),arguments)};b.wbg.__wbg_scrollHeight_2d7f990052968501=(a=>{const b=c(a).scrollHeight;return b});b.wbg.__wbg_focus_39d4b8ba8ff9df14=function(){return C((a=>{c(a).focus()}),arguments)};b.wbg.__wbg_target_2fc177e386c8b7b0=(a=>{const b=c(a).target;return B(b)?T:k(b)});b.wbg.__wbg_bubbles_abce839854481bc6=(a=>{const b=c(a).bubbles;return b});b.wbg.__wbg_cancelBubble_c0aa3172524eb03c=(a=>{const b=c(a).cancelBubble;return b});b.wbg.__wbg_composedPath_58473fd5ae55f2cd=(a=>{const b=c(a).composedPath();return k(b)});b.wbg.__wbg_preventDefault_b1a4aafc79409429=(a=>{c(a).preventDefault()});b.wbg.__wbg_instanceof_KeyboardEvent_d51b1a079e0c6a46=(a=>{let b;try{b=c(a) instanceof KeyboardEvent}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_ctrlKey_bb5b6fef87339703=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_5911baf439ab232b=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_metaKey_6bf4ae4e83a11278=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_key_dccf9e8aa1315a8e=((b,d)=>{const e=c(d).key;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbg_addEventListener_4283b15b4f039eb5=function(){return C(((a,b,d,e,f)=>{c(a).addEventListener(j(b,d),c(e),c(f))}),arguments)};b.wbg.__wbg_removeEventListener_5d31483804421bfa=function(){return C(((a,b,d,e,f)=>{c(a).removeEventListener(j(b,d),c(e),f!==T)}),arguments)};b.wbg.__wbg_setchecked_931ff2ed2cd3ebfd=((a,b)=>{c(a).checked=b!==T});b.wbg.__wbg_value_47fe6384562f52ab=((b,d)=>{const e=c(d).value;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbg_setvalue_78cb4f1fef58ae98=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_value_d7f5bfbd9302c14b=((b,d)=>{const e=c(d).value;const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbg_setvalue_090972231f0a4f6f=((a,b,d)=>{c(a).value=j(b,d)});b.wbg.__wbg_parentNode_6be3abff20e1a5fb=(a=>{const b=c(a).parentNode;return B(b)?T:k(b)});b.wbg.__wbg_parentElement_347524db59fc2976=(a=>{const b=c(a).parentElement;return B(b)?T:k(b)});b.wbg.__wbg_childNodes_118168e8b23bcb9b=(a=>{const b=c(a).childNodes;return k(b)});b.wbg.__wbg_lastChild_83efe6d5da370e1f=(a=>{const b=c(a).lastChild;return B(b)?T:k(b)});b.wbg.__wbg_nextSibling_709614fdb0fb7a66=(a=>{const b=c(a).nextSibling;return B(b)?T:k(b)});b.wbg.__wbg_setnodeValue_94b86af0cda24b90=((a,b,d)=>{c(a).nodeValue=b===T?O:j(b,d)});b.wbg.__wbg_textContent_8e392d624539e731=((b,d)=>{const e=c(d).textContent;var f=B(e)?T:p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);var g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbg_cloneNode_e19c313ea20d5d1d=function(){return C((a=>{const b=c(a).cloneNode();return k(b)}),arguments)};b.wbg.__wbg_insertBefore_d2a001abf538c1f8=function(){return C(((a,b,d)=>{const e=c(a).insertBefore(c(b),c(d));return k(e)}),arguments)};b.wbg.__wbg_removeChild_96bbfefd2f5a0261=function(){return C(((a,b)=>{const d=c(a).removeChild(c(b));return k(d)}),arguments)};b.wbg.__wbg_instanceof_ShadowRoot_9db040264422e84a=(a=>{let b;try{b=c(a) instanceof ShadowRoot}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_host_c667c7623404d6bf=(a=>{const b=c(a).host;return k(b)});b.wbg.__wbg_get_bd8e338fbd5f5cc8=((a,b)=>{const d=c(a)[b>>>T];return k(d)});b.wbg.__wbg_length_cd7af8117672b8b8=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_e258087cd0daa0ea=((a,b)=>{const c=new Function(j(a,b));return k(c)});b.wbg.__wbg_call_27c0f87801dedf93=function(){return C(((a,b)=>{const d=c(a).call(c(b));return k(d)}),arguments)};b.wbg.__wbg_new_72fb9a18b5ae2624=(()=>{const a=new $();return k(a)});b.wbg.__wbg_self_ce0dbfc45cf2f5be=function(){return C((()=>{const a=self.self;return k(a)}),arguments)};b.wbg.__wbg_window_c6fb939a7f436783=function(){return C((()=>{const a=window.window;return k(a)}),arguments)};b.wbg.__wbg_globalThis_d1e6af4856ba331b=function(){return C((()=>{const a=globalThis.globalThis;return k(a)}),arguments)};b.wbg.__wbg_global_207b558942527489=function(){return C((()=>{const a=global.global;return k(a)}),arguments)};b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===O;return b});b.wbg.__wbg_from_89e3fc3ba5e6fb48=(a=>{const b=M.from(c(a));return k(b)});b.wbg.__wbg_getTime_2bc4375165f02d15=(a=>{const b=c(a).getTime();return b});b.wbg.__wbg_new0_7d84e5b2cd9fdc73=(()=>{const a=new Date();return k(a)});b.wbg.__wbg_is_010fdc0f4ab96916=((a,b)=>{const d=$.is(c(a),c(b));return d});b.wbg.__wbg_resolve_b0083a7967828ec8=(a=>{const b=Promise.resolve(c(a));return k(b)});b.wbg.__wbg_then_0c86a60e8fcfe9f6=((a,b)=>{const d=c(a).then(c(b));return k(d)});b.wbg.__wbg_set_1f9b04f170055d33=function(){return C(((a,b,d)=>{const e=Reflect.set(c(a),c(b),c(d));return e}),arguments)};b.wbg.__wbindgen_debug_string=((b,d)=>{const e=l(c(d));const f=p(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=m;r()[b/_+ U]=g;r()[b/_+ T]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new S(j(a,b))});b.wbg.__wbindgen_closure_wrapper153=((a,b,c)=>{const d=t(a,b,Y,w);return k(d)});b.wbg.__wbindgen_closure_wrapper155=((a,b,c)=>{const d=t(a,b,Y,x);return k(d)});b.wbg.__wbindgen_closure_wrapper1249=((a,b,c)=>{const d=y(a,b,588,z);return k(d)});b.wbg.__wbindgen_closure_wrapper1602=((a,b,c)=>{const d=t(a,b,710,A);return k(d)});return b});var t=((b,c,d,e)=>{const f={a:b,b:c,cnt:U,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=T;try{return e(c,f.b,...b)}finally{if(--f.cnt===T){a.__wbindgen_export_2.get(f.dtor)(c,f.b);s.unregister(f)}else{f.a=c}}};g.original=f;s.register(g,f,f);return g});var f=(a=>{const b=c(a);e(a);return b});var l=(a=>{const b=typeof a;if(b==`number`||b==`boolean`||a==P){return `${a}`};if(b==V){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==P){return `Symbol`}else{return `Symbol(${b})`}};if(b==W){const b=a.name;if(typeof b==V&&b.length>T){return `Function(${b})`}else{return `Function`}};if(M.isArray(a)){const b=a.length;let c=`[`;if(b>T){c+=l(a[T])};for(let d=U;d<b;d++){c+=`, `+ l(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>U){d=c[U]}else{return toString.call(a)};if(d==X){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return X}};if(a instanceof S){return `${a.name}: ${a.message}\n${a.stack}`};return d});let a;const b=new M(N).fill(O);b.push(O,P,!0,!1);let d=b.length;const g=typeof TextDecoder!==Q?new TextDecoder(R,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw S(`TextDecoder not available`)}};if(typeof TextDecoder!==Q){g.decode()};let h=P;let m=T;const n=typeof TextEncoder!==Q?new TextEncoder(R):{encode:()=>{throw S(`TextEncoder not available`)}};const o=typeof n.encodeInto===W?((a,b)=>n.encodeInto(a,b)):((a,b)=>{const c=n.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=P;const s=typeof Z===Q?{register:()=>{},unregister:()=>{}}:new Z(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});let u=N;let D=P;export default L;export{K as initSync}