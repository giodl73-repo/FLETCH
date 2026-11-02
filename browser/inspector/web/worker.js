import init,{inspect_registry} from './pkg/fletch_registry_inspector.js';
const started=performance.now();
try {const wasm=await init();self.onmessage=({data})=>{const start=performance.now();try{const result=inspect_registry(data.request);self.postMessage({id:data.id,revision:data.revision,result,computationMs:performance.now()-start});}catch(error){self.postMessage({id:data.id,revision:data.revision,error:String(error)});}};self.postMessage({ready:true,initializationMs:performance.now()-started});}catch(error){self.postMessage({startupError:String(error)});}
