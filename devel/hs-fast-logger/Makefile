# $FreeBSD$

PORTNAME=	fast-logger
PORTVERSION=	2.4.11
PORTREVISION=	2
CATEGORIES=	devel haskell

MAINTAINER=	haskell@FreeBSD.org
COMMENT=	Fast logging system

LICENSE=	BSD3CLAUSE

DEPRECATED=		Haskell libraries are getting pushed out of the ports tree. If you are using this port for development, consider switching to cabal sandboxes (old way), cabal new-build (new way), or use devel/stack. If you believe this port shouldn't be removed, contact haskell@FreeBSD.org
EXPIRATION_DATE=	2019-04-14

USE_CABAL=	auto-update easy-file old-locale unix-time

.include "${.CURDIR}/../../lang/ghc/bsd.cabal.mk"
.include <bsd.port.mk>
