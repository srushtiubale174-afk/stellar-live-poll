import { useState, useEffect } from 'react';
import {
  StellarWalletsKit,
  WalletNetwork,
  allowAllModules,
} from '@creit.tech/stellar-wallets-kit';
import {
  Contract,
  TransactionBuilder,
  Networks,
  BASE_FEE,
  rpc,
  scValToNative,
  nativeToScVal,
  Address,
} from '@stellar/stellar-sdk';
import './App.css';

const CONTRACT_ID = 'CCCGQQORMNCIJDSXWVHT2A5GBP5F2WQ6LELO4GTHYSYTM53AMNVDE7ET';
const RPC_URL = 'https://soroban-testnet.stellar.org';
const NETWORK_PASSPHRASE = Networks.TESTNET;

const OPTIONS = ['Stellar', 'Ethereum', 'Solana'];

const kit = new StellarWalletsKit({
  network: WalletNetwork.TESTNET,
  selectedWalletId: 'freighter',
  modules: allowAllModules(),
});

function App() {
  const [address, setAddress] = useState(null);
  const [results, setResults] = useState({});
  const [txStatus, setTxStatus] = useState('idle');
  const [errorMsg, setErrorMsg] = useState('');
  const [lastTxHash, setLastTxHash] = useState('');

  const connectWallet = async () => {
    setErrorMsg('');
    try {
      await kit.openModal({
        onWalletSelected: async (option) => {
          kit.setWallet(option.id);
          const result = await kit.getAddress();
          setAddress(result.address);
        },
      });
    } catch (err) {
      if (err.message && err.message.includes('not installed')) {
        setErrorMsg('Wallet nahi mila. Please Freighter wallet install karo.');
      } else {
        setErrorMsg('Wallet connect nahi ho paya: ' + err.message);
      }
    }
  };

  const fetchResults = async () => {
    try {
      const server = new rpc.Server(RPC_URL);
      const contract = new Contract(CONTRACT_ID);
      const account = await server.getAccount(
        address || 'GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF'
      );

      const tx = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(contract.call('get_results'))
        .setTimeout(30)
        .build();

      const sim = await server.simulateTransaction(tx);
      if (sim.result) {
        const native = scValToNative(sim.result.retval);
        setResults(native);
      }
    } catch (err) {
      console.error('Fetch results error:', err);
    }
  };

  const vote = async (option) => {
    if (!address) {
      setErrorMsg('Pehle wallet connect karo.');
      return;
    }
    setErrorMsg('');
    setTxStatus('pending');
    setLastTxHash('');

    try {
      const server = new rpc.Server(RPC_URL);
      const contract = new Contract(CONTRACT_ID);
      const account = await server.getAccount(address);

      let tx = new TransactionBuilder(account, {
        fee: BASE_FEE,
        networkPassphrase: NETWORK_PASSPHRASE,
      })
        .addOperation(
          contract.call(
            'vote',
            Address.fromString(address).toScVal(),
            nativeToScVal(option, { type: 'symbol' })
          )
        )
        .setTimeout(30)
        .build();

      const sim = await server.simulateTransaction(tx);

      if (rpc.Api.isSimulationError(sim)) {
        console.error('Simulation error:', sim.error);
        if (sim.error && (sim.error.includes('trustline') || sim.error.includes('balance'))) {
          setErrorMsg('Insufficient balance transaction fees ke liye.');
        } else {
          setErrorMsg('Contract error: ' + sim.error);
        }
        setTxStatus('fail');
        return;
      }

      const preparedTx = rpc.assembleTransaction(tx, sim).build();
      const xdr = preparedTx.toXDR();

      let signedXdr;
      try {
        const result = await kit.signTransaction(xdr, {
          networkPassphrase: NETWORK_PASSPHRASE,
        });
        signedXdr = result.signedTxXdr;
      } catch (err) {
        console.error('Sign error:', err);
        setErrorMsg('Transaction reject kar diya gaya user dwara.');
        setTxStatus('fail');
        return;
      }

      const signedTx = TransactionBuilder.fromXDR(signedXdr, NETWORK_PASSPHRASE);
      const sendResult = await server.sendTransaction(signedTx);

      if (sendResult.status === 'ERROR') {
        console.error('Send error:', sendResult);
        setErrorMsg('Transaction bhejne mein error aaya.');
        setTxStatus('fail');
        return;
      }

      setLastTxHash(sendResult.hash);

      let status = 'PENDING';
      let attempts = 0;
      while (status === 'PENDING' && attempts < 15) {
        await new Promise(function (r) {
          setTimeout(r, 2000);
        });
        const check = await server.getTransaction(sendResult.hash);
        status = check.status;
        attempts++;
      }

      if (status === 'SUCCESS') {
        setTxStatus('success');
        fetchResults();
      } else {
        console.error('Final tx status:', status);
        setTxStatus('fail');
        setErrorMsg('Transaction fail ho gaya.');
      }
    } catch (err) {
      console.error('Vote catch error:', err);
      setTxStatus('fail');
      setErrorMsg('Kuch galat ho gaya: ' + err.message);
    }
  };

  useEffect(function () {
    fetchResults();
  }, []);

  return (
    <div className="app">
      <h1>Live Poll: Best Blockchain?</h1>

      {!address && (
        <button onClick={connectWallet} className="connect-btn">
          Connect Wallet
        </button>
      )}

      {address && (
        <p className="address">
          Connected: {address.slice(0, 6)}...{address.slice(-6)}
        </p>
      )}

      {errorMsg && <p className="error">{errorMsg}</p>}

      <div className="options">
        {OPTIONS.map(function (option) {
          return (
            <div key={option} className="option-row">
              <button onClick={function () { vote(option); }} disabled={txStatus === 'pending'}>
                Vote for {option}
              </button>
              <span className="count">{results[option] || 0} votes</span>
            </div>
          );
        })}
      </div>

      {txStatus === 'pending' && <p className="status pending">Transaction pending...</p>}

      {txStatus === 'success' && (
        <p className="status success">
          Vote successful! Check the explorer link below.
          <br />
          <a href={'https://stellar.expert/explorer/testnet/tx/' + lastTxHash} target="_blank" rel="noreferrer">
            View on Explorer
          </a>
        </p>
      )}

      {txStatus === 'fail' && <p className="status fail">Transaction failed.</p>}
    </div>
  );
}

export default App;