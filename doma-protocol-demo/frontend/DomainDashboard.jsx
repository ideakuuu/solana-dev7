import React, { useEffect, useState } from 'react';
import domains from '../sample-data/domains.json';

export default function DomainDashboard(){
  const [data, setData] = useState([]);

  useEffect(()=> {
    setData(domains);
  }, []);

  return (
    <div>
      <h2>Sample Domains</h2>
      <table border="1" cellPadding="6">
        <thead><tr><th>Domain</th><th>Token ID</th><th>Owner</th><th>Metadata</th></tr></thead>
        <tbody>
          {data.map(d => (
            <tr key={d.tokenId}><td>{d.name}</td><td>{d.tokenId}</td><td>{d.owner}</td><td><a href={d.metadataUri} target="_blank" rel="noreferrer">view</a></td></tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
