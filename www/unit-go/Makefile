# Created by: Li-Wen Hsu <lwhsu@FreeBSD.org>
# $FreeBSD$

PKGNAMESUFFIX=	-go

COMMENT=	Go module for NGINX Unit

DEPRECATED=		Go libraries should not be packaged
EXPIRATION_DATE=	2019-03-31

USES=		go

PLIST_FILES=	# reset to empty
PLIST_DIRS=	# reset to empty
PLIST=		${.CURDIR}/pkg-plist

USE_RC_SUBR?=	# reset to empty

MASTERDIR=	${.CURDIR}/../unit

CGO_CFLAGS+=	-I${WRKSRC}/src -I${WRKSRC}/build
GO_PKGNAME=	nginx/unit

post-extract:
	@true # override master's

post-configure:
	cd ${WRKSRC} && ${SETENV} ${GO_ENV} GOCACHE=${WRKSRC}/go ./configure go --go-path=${WRKDIR}

pre-build:
	cd ${WRKSRC} && ${MAKE_CMD} && ${SETENV} ${GO_ENV} GOCACHE=${WRKSRC}/go ${MAKE_CMD} go-install

post-install:
	@true # override master's

.include "${MASTERDIR}/Makefile"
