# RustComm
Prototype for message-passing communication system
Provides:
- Connector&lt;P,M,L&gt;
- Listener&lt;P,L&gt; with ThreadPool<TcpStream>
- Message
- CommProcessing&lt;L&gt;

All application specific processing is in CommProcessing&lt;L&gt;.

See https://JimFawcett.github.io/RustCommWithThreadPool.html for details.
