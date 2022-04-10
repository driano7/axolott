pragma solidity 0.8.6;

contract auditoriaAldesempenioDemo {
    event LogMsg(string message);
    string public storedMsg;
    constructor() {
        storedMsg = "Este es el primer contrato inteligente de auditoria en Ethereum";
        emit LogMsg(storedMsg);
    }

function updateMsg(string memory newMsg)
    public {
       storedMsg = newMsg;
        emit LogMsg(storedMsg);
    }
}
