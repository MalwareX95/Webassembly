import _ from 'lodash';
import React, { useEffect, useMemo, useRef, useState } from 'react';
import './App.css';
import { Network, parseDOTNetwork, Data, Edge, Node, IdType} from 'vis-network/standalone'
import { dijkstra, hello_delay, parrallelSum } from './wasm-pkg/hello_wasm'

function toAdjacentyMatrix(data: Data) {
   if(!data.nodes?.length) {
     return [];
   }

   const nodes = data.nodes as Node[];
   const edges = data.edges as Edge[];

   const mapIdToIndex = new Map<IdType, number>(nodes.map(({ id }, n) => ([id!, n])) as [IdType, number][])
   
   const matrix = [...Array(nodes.length)]
      .map(_ => [...Array(nodes.length)].map(_ => Number.POSITIVE_INFINITY));
   
   for (const { from, to, value } of edges) {
      matrix[mapIdToIndex.get(from!)!][mapIdToIndex.get(to!)!] = value ?? 1;
   }

   return matrix.flat();
}


function App() {
  const dataRef = useRef<HTMLTextAreaElement>(null);
  const container = useRef<HTMLDivElement>(null);
  const [str, setStr] = useState<string>();
  const [error, setError] = useState<string>()
  
  const network = useMemo(() => 
    container.current && new Network(container.current, {})
  , [ container.current ]);

  function handleOnBlur() {
    setStr(dataRef.current?.value);
  }

  async function sum() {
    let result = await parrallelSum();
    console.log(result);
  }

  async function saySomething() {
    const str = await hello_delay('World');
    console.log(str);
  }

  useEffect(() => {
    if(network) {
      try {
        const data = parseDOTNetwork(dataRef.current?.value);
        network.setData({edges: data.edges, nodes: data.nodes})
        network.setOptions(data.options)
        console.log(data)
        setError('');
      }
      catch(e: any) {
        console.dir(e?.message);
        setError(e?.message);
      }
    }
  }, [ network, str, dataRef.current ])


  return (
    <div className="App">
      <div style={{display: 'flex', flexDirection: 'column', flex: 2}}>
        <div style={{textAlign: 'left'}}>
          <div>
            <label style={{display: 'block'}}>From</label>
            <input type="text" placeholder="Label node"/>
          </div>
          <div>
            <label style={{display: 'block'}}>To</label>
            <input type="text" placeholder="Label node" />
          </div>
        </div>
        <div style={{flex: 1}}>
          <textarea onBlur={handleOnBlur} ref={dataRef}></textarea>
          <div>{error}</div>
        </div>
        <button onClick={sum}>Say something</button>
      </div>
      <div id="container" style={{flex: 4}} ref={container}></div>
    </div>
  );
}

export default App;
