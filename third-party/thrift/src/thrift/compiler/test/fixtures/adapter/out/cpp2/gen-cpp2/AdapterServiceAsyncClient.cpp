/**
 * Autogenerated by Thrift for thrift/compiler/test/fixtures/adapter/src/module.thrift
 *
 * DO NOT EDIT UNLESS YOU ARE SURE THAT YOU KNOW WHAT YOU ARE DOING
 *  @generated @nocommit
 */

#include "thrift/compiler/test/fixtures/adapter/gen-cpp2/AdapterServiceAsyncClient.h"

#include <thrift/lib/cpp2/gen/client_cpp.h>

namespace facebook::thrift::test {
typedef apache::thrift::ThriftPresult<false> AdapterService_count_pargs;
typedef apache::thrift::ThriftPresult<true, apache::thrift::FieldData<0, ::apache::thrift::type_class::structure, ::facebook::thrift::test::CountingStruct*>> AdapterService_count_presult;
typedef apache::thrift::ThriftPresult<false, apache::thrift::FieldData<1, ::apache::thrift::type_class::structure, ::facebook::thrift::test::HeapAllocated*, ::apache::thrift::type::adapted<::apache::thrift::test::MoveOnlyAdapter, ::apache::thrift::type::struct_t<::facebook::thrift::test::detail::HeapAllocated>>>> AdapterService_adaptedTypes_pargs;
typedef apache::thrift::ThriftPresult<true, apache::thrift::FieldData<0, ::apache::thrift::type_class::structure, ::facebook::thrift::test::HeapAllocated*, ::apache::thrift::type::adapted<::apache::thrift::test::MoveOnlyAdapter, ::apache::thrift::type::struct_t<::facebook::thrift::test::detail::HeapAllocated>>>> AdapterService_adaptedTypes_presult;
} // namespace facebook::thrift::test
template <typename Protocol_, typename RpcOptions>
void apache::thrift::Client<::facebook::thrift::test::AdapterService>::countT(Protocol_* prot, RpcOptions&& rpcOptions, std::shared_ptr<apache::thrift::transport::THeader> header, apache::thrift::ContextStack* contextStack, apache::thrift::RequestClientCallback::Ptr callback) {

  ::facebook::thrift::test::AdapterService_count_pargs args;
  auto sizer = [&](Protocol_* p) { return args.serializedSizeZC(p); };
  auto writer = [&](Protocol_* p) { args.write(p); };

  static ::apache::thrift::MethodMetadata::Data* methodMetadata =
        new ::apache::thrift::MethodMetadata::Data(
                "count",
                ::apache::thrift::FunctionQualifier::Unspecified,
                "facebook.com/thrift/test/AdapterService");
  apache::thrift::clientSendT<apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, Protocol_>(prot, std::forward<RpcOptions>(rpcOptions), std::move(callback), contextStack, std::move(header), channel_.get(), ::apache::thrift::MethodMetadata::from_static(methodMetadata), writer, sizer);
}

template <typename Protocol_, typename RpcOptions>
void apache::thrift::Client<::facebook::thrift::test::AdapterService>::adaptedTypesT(Protocol_* prot, RpcOptions&& rpcOptions, std::shared_ptr<apache::thrift::transport::THeader> header, apache::thrift::ContextStack* contextStack, apache::thrift::RequestClientCallback::Ptr callback, const ::facebook::thrift::test::HeapAllocated& p_arg) {

  ::facebook::thrift::test::AdapterService_adaptedTypes_pargs args;
  args.get<0>().value = const_cast<::facebook::thrift::test::HeapAllocated*>(&p_arg);
  auto sizer = [&](Protocol_* p) { return args.serializedSizeZC(p); };
  auto writer = [&](Protocol_* p) { args.write(p); };

  static ::apache::thrift::MethodMetadata::Data* methodMetadata =
        new ::apache::thrift::MethodMetadata::Data(
                "adaptedTypes",
                ::apache::thrift::FunctionQualifier::Unspecified,
                "facebook.com/thrift/test/AdapterService");
  apache::thrift::clientSendT<apache::thrift::RpcKind::SINGLE_REQUEST_SINGLE_RESPONSE, Protocol_>(prot, std::forward<RpcOptions>(rpcOptions), std::move(callback), contextStack, std::move(header), channel_.get(), ::apache::thrift::MethodMetadata::from_static(methodMetadata), writer, sizer);
}



void apache::thrift::Client<::facebook::thrift::test::AdapterService>::count(std::unique_ptr<apache::thrift::RequestCallback> callback) {
  ::apache::thrift::RpcOptions rpcOptions;
  count(rpcOptions, std::move(callback));
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::count(apache::thrift::RpcOptions& rpcOptions, std::unique_ptr<apache::thrift::RequestCallback> callback) {
  auto [ctx, header] = countCtx(&rpcOptions);
  apache::thrift::RequestCallback::Context callbackContext;
  callbackContext.protocolId =
      apache::thrift::GeneratedAsyncClient::getChannel()->getProtocolId();
  auto* contextStack = ctx.get();
  if (callback) {
    callbackContext.ctx = std::move(ctx);
  }
  auto wrappedCallback = apache::thrift::toRequestClientCallbackPtr(std::move(callback), std::move(callbackContext));
  countImpl(rpcOptions, std::move(header), contextStack, std::move(wrappedCallback));
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::countImpl(apache::thrift::RpcOptions& rpcOptions, std::shared_ptr<apache::thrift::transport::THeader> header, apache::thrift::ContextStack* contextStack, apache::thrift::RequestClientCallback::Ptr callback, bool stealRpcOptions) {
  switch (apache::thrift::GeneratedAsyncClient::getChannel()->getProtocolId()) {
    case apache::thrift::protocol::T_BINARY_PROTOCOL:
    {
      apache::thrift::BinaryProtocolWriter writer;
      if (stealRpcOptions) {
        countT(&writer, std::move(rpcOptions), std::move(header), contextStack, std::move(callback));
      } else {
        countT(&writer, rpcOptions, std::move(header), contextStack, std::move(callback));
      }
      break;
    }
    case apache::thrift::protocol::T_COMPACT_PROTOCOL:
    {
      apache::thrift::CompactProtocolWriter writer;
      if (stealRpcOptions) {
        countT(&writer, std::move(rpcOptions), std::move(header), contextStack, std::move(callback));
      } else {
        countT(&writer, rpcOptions, std::move(header), contextStack, std::move(callback));
      }
      break;
    }
    default:
    {
      apache::thrift::detail::ac::throw_app_exn("Could not find Protocol");
    }
  }
}

std::pair<::apache::thrift::ContextStack::UniquePtr, std::shared_ptr<::apache::thrift::transport::THeader>> apache::thrift::Client<::facebook::thrift::test::AdapterService>::countCtx(apache::thrift::RpcOptions* rpcOptions) {
  auto header = std::make_shared<apache::thrift::transport::THeader>(
      apache::thrift::transport::THeader::ALLOW_BIG_FRAMES);
  header->setProtocolId(channel_->getProtocolId());
  if (rpcOptions) {
    header->setHeaders(rpcOptions->releaseWriteHeaders());
  }

  auto ctx = apache::thrift::ContextStack::createWithClientContext(
      handlers_,
      interceptors_,
      getServiceName(),
      "AdapterService.count",
      *header);

  return {std::move(ctx), std::move(header)};
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::sync_count(::facebook::thrift::test::CountingStruct& _return) {
  ::apache::thrift::RpcOptions rpcOptions;
  sync_count(rpcOptions, _return);
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::sync_count(apache::thrift::RpcOptions& rpcOptions, ::facebook::thrift::test::CountingStruct& _return) {
  apache::thrift::ClientReceiveState returnState;
  apache::thrift::ClientSyncCallback<false> callback(&returnState);
  auto protocolId = apache::thrift::GeneratedAsyncClient::getChannel()->getProtocolId();
  auto evb = apache::thrift::GeneratedAsyncClient::getChannel()->getEventBase();
  auto ctxAndHeader = countCtx(&rpcOptions);
  auto wrappedCallback = apache::thrift::RequestClientCallback::Ptr(&callback);
  callback.waitUntilDone(
    evb,
    [&] {
      countImpl(rpcOptions, std::move(ctxAndHeader.second), ctxAndHeader.first.get(), std::move(wrappedCallback));
    });

  if (returnState.isException()) {
    returnState.exception().throw_exception();
  }
  returnState.resetProtocolId(protocolId);
  returnState.resetCtx(std::move(ctxAndHeader.first));
  SCOPE_EXIT {
    if (returnState.header() && !returnState.header()->getHeaders().empty()) {
      rpcOptions.setReadHeaders(returnState.header()->releaseHeaders());
    }
  };
  return folly::fibers::runInMainContext([&] {
      recv_count(_return, returnState);
  });
}


folly::Future<::facebook::thrift::test::CountingStruct> apache::thrift::Client<::facebook::thrift::test::AdapterService>::future_count() {
  ::apache::thrift::RpcOptions rpcOptions;
  return future_count(rpcOptions);
}

folly::SemiFuture<::facebook::thrift::test::CountingStruct> apache::thrift::Client<::facebook::thrift::test::AdapterService>::semifuture_count() {
  ::apache::thrift::RpcOptions rpcOptions;
  return semifuture_count(rpcOptions);
}

folly::Future<::facebook::thrift::test::CountingStruct> apache::thrift::Client<::facebook::thrift::test::AdapterService>::future_count(apache::thrift::RpcOptions& rpcOptions) {
  folly::Promise<::facebook::thrift::test::CountingStruct> promise;
  auto future = promise.getFuture();
  auto callback = std::make_unique<apache::thrift::FutureCallback<::facebook::thrift::test::CountingStruct>>(std::move(promise), recv_wrapped_count, channel_);
  count(rpcOptions, std::move(callback));
  return future;
}

folly::SemiFuture<::facebook::thrift::test::CountingStruct> apache::thrift::Client<::facebook::thrift::test::AdapterService>::semifuture_count(apache::thrift::RpcOptions& rpcOptions) {
  auto callbackAndFuture = makeSemiFutureCallback(recv_wrapped_count, channel_);
  auto callback = std::move(callbackAndFuture.first);
  count(rpcOptions, std::move(callback));
  return std::move(callbackAndFuture.second);
}

folly::Future<std::pair<::facebook::thrift::test::CountingStruct, std::unique_ptr<apache::thrift::transport::THeader>>> apache::thrift::Client<::facebook::thrift::test::AdapterService>::header_future_count(apache::thrift::RpcOptions& rpcOptions) {
  folly::Promise<std::pair<::facebook::thrift::test::CountingStruct, std::unique_ptr<apache::thrift::transport::THeader>>> promise;
  auto future = promise.getFuture();
  auto callback = std::make_unique<apache::thrift::HeaderFutureCallback<::facebook::thrift::test::CountingStruct>>(std::move(promise), recv_wrapped_count, channel_);
  count(rpcOptions, std::move(callback));
  return future;
}

folly::SemiFuture<std::pair<::facebook::thrift::test::CountingStruct, std::unique_ptr<apache::thrift::transport::THeader>>> apache::thrift::Client<::facebook::thrift::test::AdapterService>::header_semifuture_count(apache::thrift::RpcOptions& rpcOptions) {
  auto callbackAndFuture = makeHeaderSemiFutureCallback(recv_wrapped_count, channel_);
  auto callback = std::move(callbackAndFuture.first);
  count(rpcOptions, std::move(callback));
  return std::move(callbackAndFuture.second);
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::count(folly::Function<void (::apache::thrift::ClientReceiveState&&)> callback) {
  count(std::make_unique<apache::thrift::FunctionReplyCallback>(std::move(callback)));
}

#if FOLLY_HAS_COROUTINES
#endif // FOLLY_HAS_COROUTINES
folly::exception_wrapper apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_wrapped_count(::facebook::thrift::test::CountingStruct& _return, ::apache::thrift::ClientReceiveState& state) {
  if (state.isException()) {
    return std::move(state.exception());
  }
  if (!state.hasResponseBuffer()) {
    return folly::make_exception_wrapper<apache::thrift::TApplicationException>("recv_ called without result");
  }

  using result = ::facebook::thrift::test::AdapterService_count_presult;
  switch (state.protocolId()) {
    case apache::thrift::protocol::T_BINARY_PROTOCOL:
    {
      apache::thrift::BinaryProtocolReader reader;
      return apache::thrift::detail::ac::recv_wrapped<result>(
          &reader, state, _return);
    }
    case apache::thrift::protocol::T_COMPACT_PROTOCOL:
    {
      apache::thrift::CompactProtocolReader reader;
      return apache::thrift::detail::ac::recv_wrapped<result>(
          &reader, state, _return);
    }
    default:
    {
    }
  }
  return folly::make_exception_wrapper<apache::thrift::TApplicationException>("Could not find Protocol");
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_count(::facebook::thrift::test::CountingStruct& _return, ::apache::thrift::ClientReceiveState& state) {
  auto ew = recv_wrapped_count(_return, state);
  if (ew) {
    ew.throw_exception();
  }
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_instance_count(::facebook::thrift::test::CountingStruct& _return, ::apache::thrift::ClientReceiveState& state) {
  return recv_count(_return, state);
}

folly::exception_wrapper apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_instance_wrapped_count(::facebook::thrift::test::CountingStruct& _return, ::apache::thrift::ClientReceiveState& state) {
  return recv_wrapped_count(_return, state);
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::adaptedTypes(std::unique_ptr<apache::thrift::RequestCallback> callback, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  ::apache::thrift::RpcOptions rpcOptions;
  adaptedTypes(rpcOptions, std::move(callback), p_arg);
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::adaptedTypes(apache::thrift::RpcOptions& rpcOptions, std::unique_ptr<apache::thrift::RequestCallback> callback, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  auto [ctx, header] = adaptedTypesCtx(&rpcOptions);
  apache::thrift::RequestCallback::Context callbackContext;
  callbackContext.protocolId =
      apache::thrift::GeneratedAsyncClient::getChannel()->getProtocolId();
  auto* contextStack = ctx.get();
  if (callback) {
    callbackContext.ctx = std::move(ctx);
  }
  auto wrappedCallback = apache::thrift::toRequestClientCallbackPtr(std::move(callback), std::move(callbackContext));
  adaptedTypesImpl(rpcOptions, std::move(header), contextStack, std::move(wrappedCallback), p_arg);
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::adaptedTypesImpl(apache::thrift::RpcOptions& rpcOptions, std::shared_ptr<apache::thrift::transport::THeader> header, apache::thrift::ContextStack* contextStack, apache::thrift::RequestClientCallback::Ptr callback, const ::facebook::thrift::test::HeapAllocated& p_arg, bool stealRpcOptions) {
  switch (apache::thrift::GeneratedAsyncClient::getChannel()->getProtocolId()) {
    case apache::thrift::protocol::T_BINARY_PROTOCOL:
    {
      apache::thrift::BinaryProtocolWriter writer;
      if (stealRpcOptions) {
        adaptedTypesT(&writer, std::move(rpcOptions), std::move(header), contextStack, std::move(callback), p_arg);
      } else {
        adaptedTypesT(&writer, rpcOptions, std::move(header), contextStack, std::move(callback), p_arg);
      }
      break;
    }
    case apache::thrift::protocol::T_COMPACT_PROTOCOL:
    {
      apache::thrift::CompactProtocolWriter writer;
      if (stealRpcOptions) {
        adaptedTypesT(&writer, std::move(rpcOptions), std::move(header), contextStack, std::move(callback), p_arg);
      } else {
        adaptedTypesT(&writer, rpcOptions, std::move(header), contextStack, std::move(callback), p_arg);
      }
      break;
    }
    default:
    {
      apache::thrift::detail::ac::throw_app_exn("Could not find Protocol");
    }
  }
}

std::pair<::apache::thrift::ContextStack::UniquePtr, std::shared_ptr<::apache::thrift::transport::THeader>> apache::thrift::Client<::facebook::thrift::test::AdapterService>::adaptedTypesCtx(apache::thrift::RpcOptions* rpcOptions) {
  auto header = std::make_shared<apache::thrift::transport::THeader>(
      apache::thrift::transport::THeader::ALLOW_BIG_FRAMES);
  header->setProtocolId(channel_->getProtocolId());
  if (rpcOptions) {
    header->setHeaders(rpcOptions->releaseWriteHeaders());
  }

  auto ctx = apache::thrift::ContextStack::createWithClientContext(
      handlers_,
      interceptors_,
      getServiceName(),
      "AdapterService.adaptedTypes",
      *header);

  return {std::move(ctx), std::move(header)};
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::sync_adaptedTypes(::facebook::thrift::test::HeapAllocated& _return, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  ::apache::thrift::RpcOptions rpcOptions;
  sync_adaptedTypes(rpcOptions, _return, p_arg);
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::sync_adaptedTypes(apache::thrift::RpcOptions& rpcOptions, ::facebook::thrift::test::HeapAllocated& _return, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  apache::thrift::ClientReceiveState returnState;
  apache::thrift::ClientSyncCallback<false> callback(&returnState);
  auto protocolId = apache::thrift::GeneratedAsyncClient::getChannel()->getProtocolId();
  auto evb = apache::thrift::GeneratedAsyncClient::getChannel()->getEventBase();
  auto ctxAndHeader = adaptedTypesCtx(&rpcOptions);
  auto wrappedCallback = apache::thrift::RequestClientCallback::Ptr(&callback);
  callback.waitUntilDone(
    evb,
    [&] {
      adaptedTypesImpl(rpcOptions, std::move(ctxAndHeader.second), ctxAndHeader.first.get(), std::move(wrappedCallback), p_arg);
    });

  if (returnState.isException()) {
    returnState.exception().throw_exception();
  }
  returnState.resetProtocolId(protocolId);
  returnState.resetCtx(std::move(ctxAndHeader.first));
  SCOPE_EXIT {
    if (returnState.header() && !returnState.header()->getHeaders().empty()) {
      rpcOptions.setReadHeaders(returnState.header()->releaseHeaders());
    }
  };
  return folly::fibers::runInMainContext([&] {
      recv_adaptedTypes(_return, returnState);
  });
}


folly::Future<::facebook::thrift::test::HeapAllocated> apache::thrift::Client<::facebook::thrift::test::AdapterService>::future_adaptedTypes(const ::facebook::thrift::test::HeapAllocated& p_arg) {
  ::apache::thrift::RpcOptions rpcOptions;
  return future_adaptedTypes(rpcOptions, p_arg);
}

folly::SemiFuture<::facebook::thrift::test::HeapAllocated> apache::thrift::Client<::facebook::thrift::test::AdapterService>::semifuture_adaptedTypes(const ::facebook::thrift::test::HeapAllocated& p_arg) {
  ::apache::thrift::RpcOptions rpcOptions;
  return semifuture_adaptedTypes(rpcOptions, p_arg);
}

folly::Future<::facebook::thrift::test::HeapAllocated> apache::thrift::Client<::facebook::thrift::test::AdapterService>::future_adaptedTypes(apache::thrift::RpcOptions& rpcOptions, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  folly::Promise<::facebook::thrift::test::HeapAllocated> promise;
  auto future = promise.getFuture();
  auto callback = std::make_unique<apache::thrift::FutureCallback<::facebook::thrift::test::HeapAllocated>>(std::move(promise), recv_wrapped_adaptedTypes, channel_);
  adaptedTypes(rpcOptions, std::move(callback), p_arg);
  return future;
}

folly::SemiFuture<::facebook::thrift::test::HeapAllocated> apache::thrift::Client<::facebook::thrift::test::AdapterService>::semifuture_adaptedTypes(apache::thrift::RpcOptions& rpcOptions, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  auto callbackAndFuture = makeSemiFutureCallback(recv_wrapped_adaptedTypes, channel_);
  auto callback = std::move(callbackAndFuture.first);
  adaptedTypes(rpcOptions, std::move(callback), p_arg);
  return std::move(callbackAndFuture.second);
}

folly::Future<std::pair<::facebook::thrift::test::HeapAllocated, std::unique_ptr<apache::thrift::transport::THeader>>> apache::thrift::Client<::facebook::thrift::test::AdapterService>::header_future_adaptedTypes(apache::thrift::RpcOptions& rpcOptions, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  folly::Promise<std::pair<::facebook::thrift::test::HeapAllocated, std::unique_ptr<apache::thrift::transport::THeader>>> promise;
  auto future = promise.getFuture();
  auto callback = std::make_unique<apache::thrift::HeaderFutureCallback<::facebook::thrift::test::HeapAllocated>>(std::move(promise), recv_wrapped_adaptedTypes, channel_);
  adaptedTypes(rpcOptions, std::move(callback), p_arg);
  return future;
}

folly::SemiFuture<std::pair<::facebook::thrift::test::HeapAllocated, std::unique_ptr<apache::thrift::transport::THeader>>> apache::thrift::Client<::facebook::thrift::test::AdapterService>::header_semifuture_adaptedTypes(apache::thrift::RpcOptions& rpcOptions, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  auto callbackAndFuture = makeHeaderSemiFutureCallback(recv_wrapped_adaptedTypes, channel_);
  auto callback = std::move(callbackAndFuture.first);
  adaptedTypes(rpcOptions, std::move(callback), p_arg);
  return std::move(callbackAndFuture.second);
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::adaptedTypes(folly::Function<void (::apache::thrift::ClientReceiveState&&)> callback, const ::facebook::thrift::test::HeapAllocated& p_arg) {
  adaptedTypes(std::make_unique<apache::thrift::FunctionReplyCallback>(std::move(callback)), p_arg);
}

#if FOLLY_HAS_COROUTINES
#endif // FOLLY_HAS_COROUTINES
folly::exception_wrapper apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_wrapped_adaptedTypes(::facebook::thrift::test::HeapAllocated& _return, ::apache::thrift::ClientReceiveState& state) {
  if (state.isException()) {
    return std::move(state.exception());
  }
  if (!state.hasResponseBuffer()) {
    return folly::make_exception_wrapper<apache::thrift::TApplicationException>("recv_ called without result");
  }

  using result = ::facebook::thrift::test::AdapterService_adaptedTypes_presult;
  switch (state.protocolId()) {
    case apache::thrift::protocol::T_BINARY_PROTOCOL:
    {
      apache::thrift::BinaryProtocolReader reader;
      return apache::thrift::detail::ac::recv_wrapped<result>(
          &reader, state, _return);
    }
    case apache::thrift::protocol::T_COMPACT_PROTOCOL:
    {
      apache::thrift::CompactProtocolReader reader;
      return apache::thrift::detail::ac::recv_wrapped<result>(
          &reader, state, _return);
    }
    default:
    {
    }
  }
  return folly::make_exception_wrapper<apache::thrift::TApplicationException>("Could not find Protocol");
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_adaptedTypes(::facebook::thrift::test::HeapAllocated& _return, ::apache::thrift::ClientReceiveState& state) {
  auto ew = recv_wrapped_adaptedTypes(_return, state);
  if (ew) {
    ew.throw_exception();
  }
}

void apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_instance_adaptedTypes(::facebook::thrift::test::HeapAllocated& _return, ::apache::thrift::ClientReceiveState& state) {
  return recv_adaptedTypes(_return, state);
}

folly::exception_wrapper apache::thrift::Client<::facebook::thrift::test::AdapterService>::recv_instance_wrapped_adaptedTypes(::facebook::thrift::test::HeapAllocated& _return, ::apache::thrift::ClientReceiveState& state) {
  return recv_wrapped_adaptedTypes(_return, state);
}


