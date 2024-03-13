/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

package thrift

import (
	"fmt"
)

type HeaderProtocol struct {
	Protocol
	origTransport Transport
	trans         *HeaderTransport

	protoID ProtocolID
}

type HeaderProtocolFactory struct{}

func NewHeaderProtocolFactory() *HeaderProtocolFactory {
	return &HeaderProtocolFactory{}
}

func (p *HeaderProtocolFactory) GetProtocol(trans Transport) Protocol {
	return NewHeaderProtocol(trans)
}

func NewHeaderProtocol(trans Transport) *HeaderProtocol {
	p := &HeaderProtocol{
		origTransport: trans,
		protoID:       ProtocolIDCompact,
	}
	if et, ok := trans.(*HeaderTransport); ok {
		p.trans = et
	} else {
		p.trans = NewHeaderTransport(trans)
	}

	// Effectively an invariant violation.
	if err := p.ResetProtocol(); err != nil {
		panic(err)
	}
	return p
}

func (p *HeaderProtocol) ResetProtocol() error {
	if p.Protocol != nil && p.protoID == p.trans.ProtocolID() {
		return nil
	}

	p.protoID = p.trans.ProtocolID()
	switch p.protoID {
	case ProtocolIDBinary:
		// These defaults match cpp implementation
		p.Protocol = NewBinaryProtocol(p.trans, false, true)
	case ProtocolIDCompact:
		p.Protocol = NewCompactProtocol(p.trans)
	default:
		return NewProtocolException(fmt.Errorf("Unknown protocol id: %#x", p.protoID))
	}
	return nil
}

//
// Writing methods.
//

func (p *HeaderProtocol) WriteMessageBegin(name string, typeId MessageType, seqid int32) error {
	p.ResetProtocol()

	// The conditions here only match on the Go client side.
	// If we are a client, set header seq id same as msg id
	if typeId == CALL || typeId == ONEWAY {
		p.trans.SetSeqID(uint32(seqid))
	}
	return p.Protocol.WriteMessageBegin(name, typeId, seqid)
}

//
// Reading methods.
//

func (p *HeaderProtocol) ReadMessageBegin() (name string, typeId MessageType, seqid int32, err error) {
	if typeId == INVALID_MESSAGE_TYPE {
		if err = p.trans.ResetProtocol(); err != nil {
			return name, EXCEPTION, seqid, err
		}
	}

	err = p.ResetProtocol()
	if err != nil {
		return name, EXCEPTION, seqid, err
	}

	// see https://github.com/apache/thrift/blob/master/doc/specs/SequenceNumbers.md
	// TODO:  This is a bug. if we are speaking header protocol, we should be using
	// seq id from the header. However, doing it here creates a non-backwards
	// compatible code between client and server, since they both use this code.
	return p.Protocol.ReadMessageBegin()
}

func (p *HeaderProtocol) Flush() (err error) {
	return NewProtocolException(p.trans.Flush())
}

func (p *HeaderProtocol) Skip(fieldType Type) (err error) {
	return SkipDefaultDepth(p, fieldType)
}

func (p *HeaderProtocol) Close() error {
	return p.origTransport.Close()
}

// Deprecated: SetSeqID() is a deprecated method.
func (p *HeaderProtocol) SetSeqID(seq uint32) {
	p.trans.SetSeqID(seq)
}

// Deprecated: GetSeqID() is a deprecated method.
func (p *HeaderProtocol) GetSeqID() uint32 {
	return p.trans.SeqID()
}

// Control underlying header transport

func (p *HeaderProtocol) SetIdentity(identity string) {
	p.trans.SetIdentity(identity)
}

func (p *HeaderProtocol) Identity() string {
	return p.trans.Identity()
}

func (p *HeaderProtocol) peerIdentity() string {
	return p.trans.peerIdentity()
}

func (p *HeaderProtocol) SetPersistentHeader(key, value string) {
	p.trans.SetPersistentHeader(key, value)
}

func (p *HeaderProtocol) GetPersistentHeader(key string) (string, bool) {
	return p.trans.GetPersistentHeader(key)
}

func (p *HeaderProtocol) GetPersistentHeaders() map[string]string {
	return p.trans.GetPersistentHeaders()
}

func (p *HeaderProtocol) ClearPersistentHeaders() {
	p.trans.ClearPersistentHeaders()
}

// GetRequestHeader returns a request header if the key exists, otherwise false
func (p *HeaderProtocol) GetRequestHeader(key string) (string, bool) {
	return p.trans.GetRequestHeader(key)
}

// Deprecated SetHeader is deprecated, rather use SetRequestHeader
func (p *HeaderProtocol) SetHeader(key, value string) {
	p.trans.SetRequestHeader(key, value)
}

// Deprecated Header is deprecated, rather use GetRequestHeader
func (p *HeaderProtocol) Header(key string) (string, bool) {
	return p.trans.GetRequestHeader(key)
}

// Deprecated Headers is deprecated, rather use GetRequestHeaders
func (p *HeaderProtocol) Headers() map[string]string {
	return p.trans.GetRequestHeaders()
}

// Deprecated: SetRequestHeader is deprecated and will eventually be private.
func (p *HeaderProtocol) SetRequestHeader(key, value string) {
	p.trans.SetRequestHeader(key, value)
}

// Deprecated: GetRequestHeader is deprecated and will eventually be private.
func (p *HeaderProtocol) GetRequestHeaders() map[string]string {
	return p.trans.GetRequestHeaders()
}

func (p *HeaderProtocol) GetResponseHeader(key string) (string, bool) {
	return p.trans.GetResponseHeader(key)
}

func (p *HeaderProtocol) GetResponseHeaders() map[string]string {
	return p.trans.GetResponseHeaders()
}

func (p *HeaderProtocol) ProtocolID() ProtocolID {
	return p.protoID
}

// Deprecated: GetFlags() is a deprecated method.
func (t *HeaderProtocol) GetFlags() HeaderFlags {
	return t.trans.GetFlags()
}

// Deprecated: SetFlags() is a deprecated method.
func (p *HeaderProtocol) SetFlags(flags HeaderFlags) {
	p.trans.SetFlags(flags)
}

func (p *HeaderProtocol) AddTransform(trans TransformID) error {
	return p.trans.AddTransform(trans)
}

// Deprecated: HeaderProtocolFlags is a deprecated type, temporarily introduced to ease transition to new API.
type HeaderProtocolFlags interface {
	GetFlags() HeaderFlags
	SetFlags(flags HeaderFlags)
}

// Deprecated: HeaderProtocolProtocolID is a deprecated type, temporarily introduced to ease transition to new API.
type HeaderProtocolProtocolID interface {
	ProtocolID() ProtocolID
}
